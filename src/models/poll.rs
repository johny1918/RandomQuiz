use crate::models::vote::VoteResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Poll {
    pub id: i32,
    pub question_text: String,
    pub option_a: String,
    pub option_b: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PollResults {
    pub poll_id: i32,
    pub results: Vec<VoteResult>,
}
