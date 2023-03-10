use crate::schema::{skill_frequencies, skills, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// there is a disturbing amount of boilerplate here
// that makes it annoying to add new fields to the models
// you have to add them to the struct, the table, and the form
// and then you have to add them to the From impls

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
    // preferred output json format for the frontend
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClientUserWithSkillsForm {
    // preferred output json format for the frontend
    pub name: String,
    pub company: String,
    pub email: String,
    pub phone: String,
    pub skills: Vec<SkillsForm>,
}

impl From<UserWithSkills> for ClientUserWithSkillsForm {
    fn from(user: UserWithSkills) -> Self {
        ClientUserWithSkillsForm {
            name: user.name,
            company: user.company,
            email: user.email,
            phone: user.phone,
            skills: user.skills.into_iter().map(|s| s.into()).collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserForm {
    // preferred update json format to the database
    pub name: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl UserForm {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.company.is_none()
            && self.email.is_none()
            && self.phone.is_none()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, AsChangeset, Insertable)]
#[diesel(table_name = skills)]
pub struct SkillsForm {
    // preferred update json format to the database
    pub name: String,
    pub rating: i32,
}

impl From<Skill> for SkillsForm {
    fn from(skill: Skill) -> Self {
        SkillsForm {
            name: skill.skill,
            rating: skill.rating,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserWithSkillsForm {
    // preferred input json format for the frontend
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
    // preferred input json format to the database
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
    // preferred input json format to the database
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
        NewSkill {
            user_id: -1, // make compiler happy, we'll set it later
            name: data.name,
            rating: data.rating,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Queryable, Identifiable)]
#[diesel(table_name = skill_frequencies)]
#[diesel(primary_key(name))]
pub struct SkillFrequency {
    // preferred output json format for the frontend
    pub name: String,
    pub frequency: i32,
}
