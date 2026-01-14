use crate::models::{Category, ModeratorFlags, Problem, Status};
use chrono::NaiveDateTime;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use utils::db::schema::{problem, problem_like};

#[derive(Debug, QueryableByName, Deserialize, Serialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SolutionWithOverview {
    #[diesel(sql_type = BigInt)]
    pub id: i64,

    #[diesel(sql_type = BigInt)]
    pub problem_id: i64,

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

    #[diesel(sql_type = Nullable<Text>)]
    pub email: Option<String>,

    #[diesel(sql_type = Nullable<Text>)]
    pub picture: Option<String>,

    #[diesel(sql_type = Nullable<Text>)]
    pub name: Option<String>,

}

#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = problem)]
pub struct AddProblem {
    pub anonymous: bool,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub sub_category: Option<String>,
    pub public: bool,
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel( table_name = problem_like)]
pub struct AddProblemView {
    pub problem_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProblem {
    pub anonymous: Option<bool>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub flag: Option<ModeratorFlags>,
    pub category: Option<Category>,
    pub sub_category: Option<String>,
    pub status: Option<Status>,
    pub public: Option<bool>,
    pub archive: Option<bool>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemAndSolutions {
    pub problem: ProblemWithUserOverview,
    pub solution: Vec<SolutionWithOverview>,
}

impl ProblemAndSolutions {
    pub fn new(problem: ProblemWithUserOverview, solution: Vec<SolutionWithOverview>) -> Self {
        Self { problem, solution }
    }
}

#[derive(Debug, QueryableByName, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProblemWithUserOverview {
    #[diesel(sql_type = BigInt)]
    pub id: i64,

    #[diesel(sql_type = Bool)]
    pub anonymous: bool,

    #[diesel(sql_type = Text)]
    pub title: String,

    #[diesel(sql_type = Text)]
    pub description: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub flag: Option<String>,

    #[diesel(sql_type = Nullable<BigInt>)]
    pub featured_id: Option<i64>,

    #[diesel(sql_type = Text)]
    pub category: String,

    #[diesel(sql_type = Text)]
    pub status: String,

    #[diesel(sql_type = Bool)]
    pub public: bool,

    #[diesel(sql_type = Bool)]
    pub archive: bool,

    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,

    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,

    #[diesel(sql_type = Nullable<Text>)]
    pub email: Option<String>,

    #[diesel(sql_type = Nullable<Text>)]
    pub picture: Option<String>,

    #[diesel(sql_type = Nullable<Text>)]
    pub name: Option<String>,

    #[diesel(sql_type = BigInt)]
    pub upvotes: i64,

    #[diesel(sql_type = BigInt)]
    pub downvotes: i64,

    #[diesel(sql_type = BigInt)]
    pub solution_count: i64,
}

impl UpdateProblem {
    pub fn populate_problem(&self, problem: &mut Problem) {
        if let Some(value) = self.archive {
            problem.archive = value
        }

        if let Some(value) = self.anonymous {
            problem.anonymous = value
        }

        if let Some(value) = self.public {
            problem.public = value
        }

        if let Some(value) = self.category {
            problem.category = value
        }

        if let Some(value) = self.description.clone() {
            problem.description = value
        }
        if let Some(value) = self.flag {
            problem.flag = value
        }

        if let Some(value) = self.status {
            problem.status = value
        }

        problem.sub_category = self.sub_category.clone();

        if let Some(value) = self.title.clone() {
            problem.title = value
        }

        if let Some(value) = self.user_id {
            problem.user_id = value
        }
    }
}
