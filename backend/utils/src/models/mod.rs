use crate::db::schema::oauth_requests;
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

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = oauth_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OauthRequests {
    pub id: i64,
    pub pkce_challenge: String,
    pub pkce_verifier: String,
    pub csrf_state: String,
    pub created_at: NaiveDateTime,
}

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
