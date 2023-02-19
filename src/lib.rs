use self::models::{NewUser, User};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv;
use models::{NewSkill, UserForm};

pub mod models;
pub mod schema;

const DEFAULT_PORT: u32 = 8080;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u32,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
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

pub fn get_user(conn: &mut SqliteConnection, id: i32) -> User {
    schema::users::table
        .find(id)
        .first::<User>(conn)
        .expect("Error loading user")
}

pub fn get_users(conn: &mut SqliteConnection) -> Vec<User> {
    schema::users::table
        .load::<User>(conn)
        .expect("Error loading users")
}

pub fn update_user(conn: &mut SqliteConnection, id: i32, user: UserForm) -> User {
    diesel::update(schema::users::table.find(id))
        .set(&user)
        .get_result(conn)
        .expect("Error updating user")
}

pub fn create_skills(conn: &mut SqliteConnection, skills: Vec<NewSkill>) {
    diesel::insert_into(schema::skills::table)
        .values(&skills)
        .execute(conn)
        .expect("Error saving new skills");
}
