// setup code goes here
use diesel::SqliteConnection;
use htn_backend::{establish_connection, Config};

pub fn setup() -> SqliteConnection {
    // only test after the database is created
    // and initial migrations are run
    let config = Config::init();
    let conn = establish_connection(&config);
    conn
}
