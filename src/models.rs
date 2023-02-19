use crate::schema::{skills, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// there is a disturbing amount of boilerplate here
// that makes it annoying to add new fields to the models
// you have to add them to the struct, the table, and the form
// TODO: figure out how to reduce it

#[derive(Debug, Deserialize, Serialize, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
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
#[table_name = "users"]
pub struct UserForm {
    pub name: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, AsChangeset)]
#[table_name = "skills"]
pub struct SkillsForm {
    pub user_id: Option<i32>,
    pub skill: Option<String>,
    pub rating: Option<i32>,
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

#[derive(Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = skills)]
pub struct NewSkill<'b> {
    pub user_id: i32,
    pub skill: &'b str,
    pub rating: i32,
}

impl<'a> From<&'a Skill> for NewSkill<'a> {
    fn from(skill: &'a Skill) -> Self {
        NewSkill {
            user_id: skill.user_id,
            skill: &skill.skill,
            rating: skill.rating,
        }
    }
}

impl<'a> TryInto<NewSkill<'a>> for &'a SkillsForm {
    type Error = &'static str;

    fn try_into(self) -> Result<NewSkill<'a>, Self::Error> {
        if self.user_id.is_none() {
            return Err("user_id is required");
        }

        if self.skill.is_none() {
            return Err("skill is required");
        }

        if self.rating.is_none() {
            return Err("rating is required");
        }

        // unwrap is safe because we checked for None above
        Ok(NewSkill {
            user_id: self.user_id.unwrap(),
            skill: self.skill.as_ref().unwrap(),
            rating: self.rating.unwrap(),
        })
    }
}
