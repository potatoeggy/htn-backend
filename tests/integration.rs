// integration tests go here
mod common;

use serde::{self, Deserialize};
use serde_json;
// TODO: impl registrations
// TODO: tests
// TODO: scan thingy endpoints
// this means that given a QR code, register it to a user
// this means that given a QR code, return the user it's registered to
// like a foreign key
// things to scan include:
// - contact / socials

#[derive(Deserialize)]
pub struct JsonSkill {
    #[serde(rename = "skill")] // why >:(
    name: String, // skill name, e.g., "Rust"
    rating: i32,
}

#[derive(Deserialize)]
pub struct JsonUser {
    name: String,
    company: String,
    email: String,
    phone: String,
    skills: Vec<JsonSkill>,
}

#[cfg(test)]
#[test]
pub fn init_database_matches_json() -> serde_json::Result<()> {
    let data: Vec<JsonUser> = serde_json::from_str(include_str!("../src/data/users.json"))?;
    let mut conn = common::setup();

    assert_eq!(1, 1);
    Ok(())
}

pub fn one_user_matches_correct_user() -> () {}

pub fn skill_frequency_matches_json() -> () {}

pub fn min_freq_filter_test() -> () {}

pub fn max_freq_filter_test() -> () {}

pub fn min_max_freq_filter_test() -> () {}

pub fn invalid_user_404_test() -> () {}

pub fn bad_request_400_test_user() -> () {}

pub fn bad_request_400_test_skills() -> () {}
