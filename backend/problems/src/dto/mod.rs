use crate::models::{Category, ModeratorFlags, Problem, Status};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utils::db::schema::problem;

#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = problem)]
pub struct AddProblem {
    pub anonymous: bool,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub sub_category: Option<String>,
    pub public: bool,
    pub user_id: Option<i64>,
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

#[derive(Debug, Queryable, Serialize)]
pub struct ProblemWithUserOverview {
    pub id: i64,
    pub anonymous: bool,
    pub title: String,
    pub description: String,
    pub flag: ModeratorFlags,
    pub featured_id: Option<i64>,
    pub category: Category,
    pub status: Status,
    pub public: bool,
    pub archive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_email: Option<String>,
    pub user_picture: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ProblemWithUser {
    pub id: i64,
    pub anonymous: bool,
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
    pub user_email: Option<String>,
    pub user_picture: Option<String>,
    pub user_name: Option<String>,
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
