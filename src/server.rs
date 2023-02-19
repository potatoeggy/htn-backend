use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use htn_backend::establish_connection;
use htn_backend::models::NewSkill;
use htn_backend::models::Skill;
use htn_backend::models::SkillsForm;
use htn_backend::models::User;
use htn_backend::models::UserForm;
use htn_backend::models::UserWithSkills;
use htn_backend::models::UserWithSkillsForm;
use htn_backend::schema::skills;
use htn_backend::schema::users;
use htn_backend::update_user;
use htn_backend::utils::partition_try;
use itertools::{Either, Itertools};
use serde::Deserialize;
use tide::prelude::*;
use tide::Next;
use tide::Request;

use htn_backend::Config;

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

async fn users_get(req: Request<Config>) -> tide::Result {
    let config = req.state();
    let conn = &mut establish_connection(&config);
    let users: Vec<(User, Option<Skill>)> = users::table
        .left_join(skills::table)
        .load::<(User, Option<Skill>)>(conn)
        .expect("Error loading users");

    let res = to_users_with_skills(users);
    Ok(json!(res).into())
}

async fn skills_get(req: Request<Config>) -> tide::Result {
    let config = req.state();
    let conn = &mut establish_connection(&config);
    let skills = skills::table
        .load::<Skill>(conn)
        .expect("Error loading skills");

    Ok(json!(skills).into())
}

async fn user_one_get(req: Request<Config>) -> tide::Result {
    let config = req.state();
    let conn = &mut establish_connection(&config);

    // TODO: properly handle errors (404)
    let id: i32 = req.param("id")?.parse()?;
    let user: Vec<_> = users::table
        .left_join(skills::table)
        .filter(users::id.eq(id))
        .load::<(User, Option<Skill>)>(conn)
        .expect("Error loading user");

    let res = to_users_with_skills(user);
    // there should be only one user
    Ok(json!(res.first()).into())
}

async fn user_one_put<'a>(mut req: Request<Config>) -> tide::Result {
    // TODO: properly handle errors (404)
    let data: UserWithSkillsForm = req.body_json().await?;
    let id: i32 = req.param("id")?.parse()?;
    let config = req.state();

    let (user, mut skills) = data.into();

    let conn = &mut establish_connection(&config);
    let res = update_user(conn, id, user);

    if let Some(mut skills) = skills {
        let insert_and_update = skills
            .iter()
            .map(|skill| {
                skill.user_id = Some(id);
                let res: Result<&'a NewSkill, _> = skill.try_into();
                res
            })
            .partition_map(|skill| match skill {
                Ok(skill) => Either::Left(skill),
                Err(_) => Either::Right(skill),
            });

        let res = diesel::insert_into(skills::table)
            .values(skills_insert)
            .execute(conn)
            .expect("Error inserting skills");
        println!("Inserted {} skills", res);
    }
    Ok(json!(res).into())
}

fn to_users_with_skills(data: Vec<(User, Option<Skill>)>) -> Vec<UserWithSkills> {
    let mut res: Vec<UserWithSkills> = vec![];

    let mut prev = data[0].0.clone();
    let mut current_skills: Vec<Skill> = vec![];
    for (user, skill) in data {
        if user.id != prev.id && prev.id != -1 {
            res.push(UserWithSkills::from((prev, current_skills)));
            current_skills = vec![];
        }

        if let Some(skill) = skill {
            current_skills.push(skill);
        }
        prev = user;
    }
    // push the last user
    res.push(UserWithSkills::from((prev, current_skills)));
    res
}
