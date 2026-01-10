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
pub enum LikeType {
    Up,
    Down,
}

impl ToSql<Text, Pg> for LikeType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            LikeType::Up => "Up",
            LikeType::Down => "Down",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for LikeType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Up" => Ok(LikeType::Up),
            b"Down" => Ok(LikeType::Down),
            _ => Err("Unrecognized like variant".into()),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::problem_like)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Problem, foreign_key = problem_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
// FIX DUPLICATES
pub struct ProblemLike {
    pub id: i64,
    pub option: LikeType,
    pub problem_id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::solution_like)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Solution, foreign_key = solution_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionLike {
    pub id: i64,
    pub option: LikeType,
    pub solution_id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::problem_view)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Problem, foreign_key = problem_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemView {
    pub id: i64,
    pub user_id: i64,
    pub problem_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::problem_favourite)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Problem, foreign_key = problem_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemFavourite {
    pub id: i64,
    pub user_id: i64,
    pub problem_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::solution_favourite)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Solution, foreign_key = solution_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionFavourite {
    pub id: i64,
    pub user_id: i64,
    pub solution_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
