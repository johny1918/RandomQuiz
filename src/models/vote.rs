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