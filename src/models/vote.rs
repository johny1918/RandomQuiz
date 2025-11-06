use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct VoteRequest {
    pub poll_id: i32,
    pub chosen_option: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct VoteResult {
    pub option: String,
    pub count: i64,
    pub percentage: i32,
}

#[derive(Deserialize)]
pub struct CreatePollRequest {
    pub question_text: String,
    pub option_a: String,
    pub option_b: String,
}
