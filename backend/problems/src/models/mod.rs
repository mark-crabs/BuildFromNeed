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

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Default,
)]
#[diesel(sql_type = Text)]
pub enum ModeratorFlags {
    Duplicate,
    Monitized,
    Solved,
    Irrelevant,
    Inappropriate,
    #[default]
    NotReviewed,
}

impl ToSql<Text, Pg> for ModeratorFlags {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            ModeratorFlags::Duplicate => "Duplicate",
            ModeratorFlags::Monitized => "Monitized",
            ModeratorFlags::Solved => "Solved",
            ModeratorFlags::Irrelevant => "Irrelevant",
            ModeratorFlags::Inappropriate => "Inappropriate",
            ModeratorFlags::NotReviewed => "NotReviewed",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for ModeratorFlags {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Duplicate" => Ok(ModeratorFlags::Duplicate),
            b"Monitized" => Ok(ModeratorFlags::Monitized),
            b"Solved" => Ok(ModeratorFlags::Solved),
            b"Irrelevant" => Ok(ModeratorFlags::Irrelevant),
            b"Inappropriate" => Ok(ModeratorFlags::Inappropriate),
            b"NotReviewed" => Ok(ModeratorFlags::NotReviewed),
            _ => Err("Unrecognized moderator flags variant".into()),
        }
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Default,
)]
#[diesel(sql_type = Text)]
pub enum Category {
    Technology,
    Medical,
    Sport,
    #[default]
    NotListed,
}

impl ToSql<Text, Pg> for Category {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            Category::Technology => "Technology",
            Category::Medical => "Medical",
            Category::Sport => "Sport",
            Category::NotListed => "NotListed",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Category {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Technology" => Ok(Category::Technology),
            b"Medical" => Ok(Category::Medical),
            b"Sport" => Ok(Category::Sport),
            b"NotListed" => Ok(Category::NotListed),
            _ => Err("Unrecognized category variant".into()),
        }
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Default,
)]
#[diesel(sql_type = Text)]
pub enum Status {
    Trending,
    HighDemand,
    #[default]
    Normal,
}

impl ToSql<Text, Pg> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match self {
            Status::Trending => "Trending",
            Status::HighDemand => "HighDemand",
            Status::Normal => "Normal",
        };
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Status {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Trending" => Ok(Status::Trending),
            b"HighDemand" => Ok(Status::HighDemand),
            b"Normal" => Ok(Status::Normal),
            _ => Err("Unrecognized status variant".into()),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = utils::db::schema::featured)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Featured {
    pub id: i64,
    pub expired: bool,
    pub expiring_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = utils::db::schema::problem)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Featured, foreign_key = featured_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub id: i64,
    pub anonymous: bool,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub flag: ModeratorFlags,
    pub featured_id: Option<i64>,
    pub category: Category,
    pub sub_category: Option<String>,
    pub status: Status,
    pub public: bool,
    pub archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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

#[derive(Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = utils::db::schema::solution)]
#[diesel(belongs_to(User,  foreign_key = user_id))]
#[diesel(belongs_to(Problem, foreign_key = problem_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
    pub problem_id: i64,
    pub solution_type: SolutionType,
    pub archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
