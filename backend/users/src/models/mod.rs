use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    Admin,
    Casual,
    Moderator,
}

impl ToSql<Text, Pg> for Role {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            Role::Admin => "Admin",
            Role::Casual => "Casual",
            Role::Moderator => "Moderator",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Role {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Admin" => Ok(Role::Admin),
            b"Casual" => Ok(Role::Casual),
            b"Moderator" => Ok(Role::Moderator),
            _ => Err("Unrecognized role variant".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum UserRegisterOption {
    Manual,
    Google,
}

impl ToSql<Text, Pg> for UserRegisterOption {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            UserRegisterOption::Manual => "Manual",
            UserRegisterOption::Google => "Google",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserRegisterOption {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Manual" => Ok(UserRegisterOption::Manual),
            b"Google" => Ok(UserRegisterOption::Google),
            _ => Err("Unrecognized registration option".into()),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Role,
    pub registration_option: UserRegisterOption,
    pub archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
