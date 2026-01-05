use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};
use problems::models::Problem;
use serde::{Deserialize, Serialize};
use std::io::Write;
use users::models::User;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum SolutionType {
    Solution,
    Comment,
}

impl ToSql<Text, Pg> for SolutionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            SolutionType::Solution => "Solution",
            SolutionType::Comment => "Comment",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for SolutionType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Solution" => Ok(SolutionType::Solution),
            b"Comment" => Ok(SolutionType::Comment),
            _ => Err("Unrecognized solution type variant".into()),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::solution)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Problem, foreign_key = problem_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub problem_id: i32,
    pub solution_type: SolutionType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
