pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::{NewSkill, NewUser, Skill, User, UserForm, UserWithSkills};

const DEFAULT_PORT: u32 = 8080;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u32,
    pub database_url: String,
}

impl Config {
    pub fn init() -> Self {
        dotenv::dotenv().ok();

        let port = if let Ok(port) = std::env::var("PORT") {
            port.parse::<u32>()
        } else {
            Ok(DEFAULT_PORT)
        };

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        if let Ok(port) = port {
            Config { port, database_url }
        } else {
            println!("Bad port number");
            Config {
                port: DEFAULT_PORT,
                database_url,
            }
        }
    }
}

pub fn establish_connection(config: &Config) -> SqliteConnection {
    SqliteConnection::establish(&config.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url))
}

pub fn create_user(conn: &mut SqliteConnection, user: NewUser) {
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(conn)
        .expect("Error saving new user");
}

pub fn create_users(conn: &mut SqliteConnection, users: Vec<NewUser>) {
    diesel::insert_into(schema::users::table)
        .values(&users)
        .execute(conn)
        .expect("Error saving new users");
}

pub fn update_user(conn: &mut SqliteConnection, id: i32, user: UserForm) -> Option<UserWithSkills> {
    if !user.is_empty() {
        diesel::update(schema::users::table.find(id))
            .set(&user)
            .execute(conn)
            .expect("Error updating user");
    }

    let user: Vec<_> = schema::users::table
        .left_join(schema::skills::table)
        .filter(schema::users::id.eq(id))
        .load::<(User, Option<Skill>)>(conn)
        .expect("Error loading user");

    to_users_with_skills(user).first().cloned()
}

pub fn create_skills(conn: &mut SqliteConnection, skills: Vec<NewSkill>) {
    diesel::insert_into(schema::skills::table)
        .values(&skills)
        .execute(conn)
        .expect("Error saving new skills");
}

pub fn to_users_with_skills(data: Vec<(User, Option<Skill>)>) -> Vec<UserWithSkills> {
    // convert (User, Skill)s to (User, Vec<Skill>)s
    let mut res: Vec<UserWithSkills> = vec![];

    let prev_user = data.get(0);
    if prev_user.is_none() {
        return res;
    }
    let mut prev = prev_user.unwrap().0.clone(); // mildly dangerous but we have checks
    let mut current_skills: Vec<Skill> = vec![];
    for (user, skill) in data {
        if user.id != prev.id {
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
