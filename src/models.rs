use crate::schema::{skill_frequencies, skills, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// there is a disturbing amount of boilerplate here
// that makes it annoying to add new fields to the models
// you have to add them to the struct, the table, and the form
// TODO: figure out how to reduce it

#[derive(Debug, Deserialize, Serialize, Clone, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
pub struct Skill {
    pub id: i32,
    pub user_id: i32,
    pub skill: String, // skill name, e.g., "Rust"
    pub rating: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub company: String,
    pub email: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserWithSkills {
    pub id: i32,
    pub name: String,
    pub company: String,
    pub email: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub skills: Vec<Skill>,
}

impl From<(User, Vec<Skill>)> for UserWithSkills {
    fn from((user, skills): (User, Vec<Skill>)) -> Self {
        UserWithSkills {
            id: user.id,
            name: user.name,
            company: user.company,
            email: user.email,
            phone: user.phone,
            created_at: user.created_at,
            updated_at: user.updated_at,
            skills,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserForm {
    pub name: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, AsChangeset, Insertable)]
#[diesel(table_name = skills)]
pub struct SkillsForm {
    pub name: String,
    pub rating: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserWithSkillsForm {
    pub name: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub skills: Option<Vec<SkillsForm>>,
}

impl From<UserWithSkillsForm> for (UserForm, Option<Vec<SkillsForm>>) {
    fn from(form: UserWithSkillsForm) -> Self {
        (
            UserForm {
                name: form.name,
                company: form.company,
                email: form.email,
                phone: form.phone,
            },
            form.skills,
        )
    }
}

#[derive(Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub company: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
}

impl<'a> From<&'a User> for NewUser<'a> {
    fn from(user: &'a User) -> Self {
        NewUser {
            name: &user.name,
            company: &user.company,
            email: &user.email,
            phone: &user.phone,
        }
    }
}

#[derive(Insertable, Debug, Deserialize, Serialize, Clone, AsChangeset)]
#[diesel(table_name = skills)]
pub struct NewSkill {
    pub user_id: i32,
    pub name: String,
    pub rating: i32,
}

impl From<Skill> for NewSkill {
    fn from(skill: Skill) -> Self {
        NewSkill {
            user_id: skill.user_id,
            name: skill.skill,
            rating: skill.rating,
        }
    }
}

impl From<SkillsForm> for NewSkill {
    fn from(data: SkillsForm) -> NewSkill {
        // unwrap is safe because we checked for None above
        NewSkill {
            user_id: -1,
            name: data.name,
            rating: data.rating,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Queryable, Identifiable)]
#[diesel(table_name = skill_frequencies)]
#[diesel(primary_key(name))]
pub struct SkillFrequency {
    pub name: String,
    pub frequency: i32,
}
