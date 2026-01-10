use crate::models::{User};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utils::{auth::GoogleClaims, db::schema::users, models::Role};
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub give_name: Option<String>,
    pub picture: Option<String>,
    pub role: Option<Role>,
    pub archive: Option<bool>,
}

impl UpdateUser {
    pub fn populate_user(&self, user: &mut User) {
        if let Some(value) = self.archive {
            user.archive = value
        }

        if let Some(value) = self.role {
            user.role = value;
        }
        user.give_name = self.give_name.clone();
        user.name = self.name.clone();
        user.picture = self.picture.clone();
    }
}

#[derive(Deserialize, Insertable, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct AddUser {
    pub name: Option<String>,
    pub give_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub picture: Option<String>,
    pub role: Role,
}

impl From<GoogleClaims> for AddUser {
    fn from(value: GoogleClaims) -> Self {
        Self {
            name: value.name,
            give_name: value.given_name,
            email: value.email,
            email_verified: value.email_verified,
            picture: value.picture,
            role: Role::Casual,
        }
    }
}
