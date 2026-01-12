use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
