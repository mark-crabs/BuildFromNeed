use crate::models::{Solution, SolutionType};
use diesel::prelude::Insertable;
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
