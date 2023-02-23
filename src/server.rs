use diesel::prelude::*;
use diesel::RunQueryDsl;
use htn_backend::models::ClientUserWithSkillsForm;
use htn_backend::{
    establish_connection,
    models::{NewSkill, Skill, SkillFrequency, User, UserWithSkillsForm},
    schema::{skill_frequencies, skills, users},
    to_users_with_skills, update_user, Config,
};
use serde::Deserialize;
use tide::Request;
use tide::{prelude::*, Response, StatusCode};

pub async fn start_server(config: Config) -> tide::Result<()> {
    let mut app = tide::with_state(config.clone());

    tide::log::with_level(tide::log::LevelFilter::Debug);
    app.with(tide::log::LogMiddleware::new());
    app.at("/users").get(users_get);
    app.at("/users/:id").get(user_one_get);
    app.at("/users/:id").put(user_one_put);
    app.at("/skills").get(skills_get);

    app.listen(format!("127.0.0.1:{}", config.port)).await?;
    Ok(())
}

#[derive(Deserialize)]
struct UserQueryParams {
    name: Option<String>,
    company: Option<String>,
    email: Option<String>,
    has_skill: Option<Vec<String>>,
}

async fn users_get(req: Request<Config>) -> tide::Result {
    let config = req.state();
    let params = req.query::<UserQueryParams>()?;

    let conn = &mut establish_connection(config);

    let mut query = users::table.left_join(skills::table).into_boxed();

    if let Some(name) = params.name {
        query = query.filter(users::name.like(format!("%{}%", name)));
    }

    if let Some(company) = params.company {
        query = query.filter(users::company.like(format!("%{}%", company)));
    }

    if let Some(email) = params.email {
        query = query.filter(users::email.like(format!("%{}%", email)));
    }

    if let Some(skills) = params.has_skill {
        query = query.filter(skills::name.eq_any(skills));
    }

    let users: Vec<_> = query
        .load::<(User, Option<Skill>)>(conn)
        .expect("Error loading users");

    let res: Vec<ClientUserWithSkillsForm> = to_users_with_skills(users)
        .into_iter()
        .map(|f| f.into())
        .collect();
    Ok(json!(res).into())
}

#[derive(Deserialize)]
struct SkillQueryParams {
    min_freq: Option<i32>,
    max_freq: Option<i32>,
}

async fn skills_get(req: Request<Config>) -> tide::Result {
    let config = req.state();
    let params = req.query::<SkillQueryParams>()?;

    let conn = &mut establish_connection(config);
    let res: Vec<SkillFrequency> = skill_frequencies::table
        .select(skill_frequencies::all_columns)
        .filter(skill_frequencies::frequency.ge(params.min_freq.unwrap_or(0)))
        .filter(skill_frequencies::frequency.le(params.max_freq.unwrap_or(std::i32::MAX)))
        .load::<SkillFrequency>(conn)
        .expect("Error loading skills");
    Ok(json!(res).into())
}

async fn user_one_get(req: Request<Config>) -> tide::Result {
    let config = req.state();
    let conn = &mut establish_connection(config);

    let id = req.param("id")?.parse::<i32>().ok();
    if id.is_none() {
        // 404
        return Ok(Response::new(StatusCode::NotFound));
    }

    let user: Vec<_> = users::table
        .left_join(skills::table)
        .filter(users::id.eq(id.unwrap()))
        .load::<(User, Option<Skill>)>(conn)
        .expect("Error loading user");
    if user.is_empty() {
        // 404
        return Ok(Response::new(StatusCode::NotFound));
    }

    let res: Vec<ClientUserWithSkillsForm> = to_users_with_skills(user)
        .into_iter()
        .map(|f| f.into())
        .collect();
    // there should be only one in the whole vec
    Ok(json!(res.first()).into())
}

async fn user_one_put<'a>(mut req: Request<Config>) -> tide::Result {
    let data: Result<UserWithSkillsForm, _> = req.body_json().await;
    if data.is_err() {
        // 400
        // doesn't this feel a lot like go?
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let id = req.param("id")?.parse::<i32>().ok();
    if id.is_none() {
        // 404
        return Ok(Response::new(StatusCode::NotFound));
    }

    let config = req.state();
    let conn = &mut establish_connection(config);

    let user: Option<User> = users::table.find(id.unwrap()).first::<User>(conn).ok();

    if user.is_none() {
        // 404
        return Ok(Response::new(StatusCode::NotFound));
    }

    let (user, skills) = data.unwrap().into();
    if let Some(skills) = skills {
        let skills_insert: Vec<NewSkill> = skills
            .into_iter()
            .map(|skill| {
                let mut new_skill: NewSkill = skill.into();
                new_skill.user_id = id.unwrap();
                new_skill
            })
            .collect();

        // diesel doesn't support batched upserts so we have to
        // make a lot of queries
        // luckily usually you're not adding skills to a user

        for skill in skills_insert {
            diesel::insert_into(skills::table)
                .values(&skill)
                .on_conflict((skills::name, skills::user_id))
                .do_update()
                .set(&skill)
                .execute(conn)
                .expect("Error inserting skills");
        }
    }

    // we already checked the id so it's safe to unwrap
    let res: ClientUserWithSkillsForm = update_user(conn, id.unwrap(), user).unwrap().into();
    Ok(json!(res).into())
}
