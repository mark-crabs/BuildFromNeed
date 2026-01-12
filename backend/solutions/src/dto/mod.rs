use crate::models::{Solution, SolutionType};
use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_types::{BigInt, Bool, Text, Timestamp},
};
use serde::{Deserialize, Serialize};
use utils::db::schema::solution;
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = solution)]
pub struct AddSolution {
    pub content: String,
    pub user_id: i64,
    pub problem_id: i64,
    pub solution_type: SolutionType,
}

#[derive(Debug, QueryableByName, Deserialize, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SolutionWithOverview {
    #[diesel(sql_type = BigInt)]
    pub id: i64,

    #[diesel(sql_type = BigInt)]
    pub problem_id: i64,

    #[diesel(sql_type = BigInt)]
    pub user_id: i64,

    #[diesel(sql_type = Text)]
    pub content: String,

    #[diesel(sql_type = Bool)]
    pub archive: bool,

    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,

    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,

    #[diesel(sql_type = BigInt)]
    pub upvotes: i64,

    #[diesel(sql_type = BigInt)]
    pub downvotes: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSolution {
    pub content: Option<String>,
    pub user_id: Option<i64>,
    pub problem_id: Option<i64>,
    pub solution_type: Option<SolutionType>,
    pub archive: Option<bool>,
}

impl UpdateSolution {
    pub fn populate_solution(&self, solution: &mut Solution) {
        if let Some(value) = self.archive {
            solution.archive = value
        }

        if let Some(value) = self.content.clone() {
            solution.content = value
        }

        if let Some(value) = self.user_id {
            solution.user_id = value
        }

        if let Some(value) = self.problem_id {
            solution.problem_id = value
        }

        if let Some(value) = self.solution_type {
            solution.solution_type = value
        }
    }
}
