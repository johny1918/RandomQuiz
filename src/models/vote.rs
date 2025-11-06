use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Vote {
    pub id: i32,
    pub poll_id: i64,
    pub chosen_option: String,
    pub voted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct VoteRequest {
    pub poll_id: i32,
    pub chosen_option: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct VoteResult {
    pub option: String,
    pub count: i64,
    pub percentage: f64,
}