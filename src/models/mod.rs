use crate::models::poll::Poll;
use crate::models::state::AppState;
use crate::models::vote::VoteRequest;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use crate::models::vote::VoteResult;
use crate::models::poll::PollResults;

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

pub async fn get_results(
    State(state): State<AppState>,
    Path(poll_id): Path<i32>,
) -> Result<Json<PollResults>, (StatusCode, String)> {
    // Query for the results
    let results = sqlx::query_as::<_, VoteResult>(
        "SELECT
            chosen_option as option,
            COUNT(*) as count,
            (COUNT(*) * 100.0 / (SELECT COUNT(*) FROM votes WHERE poll_id = $1)) as percentage
         FROM votes
         WHERE poll_id = $1
         GROUP BY chosen_option",
    )   .bind(poll_id)
        .fetch_all(&state.db)  // Use fetch_all for multiple rows
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Handle case where poll has no votes
    if results.is_empty() {
        return Ok(Json(PollResults { poll_id, results: vec![] }));
    }

    Ok(Json(PollResults { poll_id, results }))
}
