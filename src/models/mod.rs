use crate::models::poll::Poll;
use crate::models::state::AppState;
use crate::models::vote::VoteRequest;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

pub mod poll;
pub mod state;
pub mod vote;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn get_random_poll(
    State(state): State<AppState>,
) -> Result<Json<Poll>, (StatusCode, String)> {
    let result = sqlx::query_as!(Poll, "SELECT * FROM polls ORDER BY RANDOM() LIMIT 1")
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "No polls available".to_string()))?;

    Ok(Json(result))
}

pub async fn submit_vote(
    State(state): State<AppState>,
    Json(vote): Json<VoteRequest>,
) -> Result<Json<&'static str>, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO votes (poll_id, chosen_option) VALUES ($1, $2)",
        vote.poll_id,
        vote.chosen_option
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json("Vote submitted successfully!"))
}
