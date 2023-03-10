// load initial sample data into the database
use htn_backend::{
    create_skills, create_users, establish_connection,
    models::{NewSkill, NewUser},
    Config,
};
use serde::Deserialize;

// the json data is not in the same format as the database
#[derive(Deserialize)]
struct JsonSkill {
    #[serde(rename = "skill")] // why >:(
    name: String, // skill name, e.g., "Rust"
    rating: i32,
}

#[derive(Deserialize)]
struct JsonUser {
    name: String,
    company: String,
    email: String,
    phone: String,
    skills: Vec<JsonSkill>,
}

fn main() -> serde_json::Result<()> {
    let data: Vec<JsonUser> = serde_json::from_str(include_str!("../data/users.json"))?;

    // WARN: there are duplicate emails in the data set
    // so we can't use the email as a unique key
    // we could do it the waterloo way and use firstnamenumberlastname
    // or the lazy way and just use the id

    let config = Config::init();
    let conn = &mut establish_connection(&config);

    let new_users: Vec<NewUser> = data
        .iter()
        .map(|user| NewUser {
            name: &user.name,
            company: &user.company,
            email: &user.email,
            phone: &user.phone,
        })
        .collect();
    create_users(conn, new_users);

    // WARN: this assumes that the users are inserted in order
    // and that these are the first users inserted
    // THEREFORE THIS WILL BREAK IF YOU RUN THIS MULTIPLE TIMES
    // OR AFTER YOU INSERT OTHER USERS
    let new_skills: Vec<NewSkill> = data
        .iter()
        .enumerate()
        .flat_map(|(i, user)| {
            user.skills
                .iter()
                .map(|skill| NewSkill {
                    user_id: i as i32 + 1, // dbs are 1-indexed
                    name: skill.name.clone(),
                    rating: skill.rating,
                })
                .collect::<Vec<NewSkill>>()
        })
        .collect();
    create_skills(conn, new_skills);

    println!("Done!");
    Ok(())
}
