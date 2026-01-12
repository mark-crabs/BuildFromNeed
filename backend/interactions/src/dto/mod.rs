use diesel::prelude::*;
use problems::models::LikeType;
use serde::{Deserialize, Serialize};
use utils::db::schema::{
    problem_favourite, problem_like, problem_view, solution_favourite, solution_like,
};

pub type ProblemDisLike = ProblemLike;

#[derive(Debug, Serialize, Deserialize)]
pub struct LikeRevoke {
    pub revoke: Option<bool>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = problem_like)]
pub struct ProblemLike {
    pub user_id: i64,
    pub problem_id: i64,
    pub option: LikeType,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = solution_like)]
pub struct SolutionLike {
    pub user_id: i64,
    pub solution_id: i64,
    pub option: LikeType,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = problem_view)]
pub struct ProblemView {
    pub user_id: i64,
    pub problem_id: i64,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = problem_favourite)]
pub struct ProblemFavourite {
    pub user_id: i64,
    pub problem_id: i64,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = solution_favourite)]
pub struct SolutionFavourite {
    pub user_id: i64,
    pub solution_id: i64,
}
