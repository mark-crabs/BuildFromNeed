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
use utils::models::Role;

#[derive(Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = utils::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub give_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub picture: Option<String>,
    pub role: Role,
    pub archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
