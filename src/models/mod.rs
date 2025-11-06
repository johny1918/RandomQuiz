use crate::models::poll::Poll;
use crate::models::poll::PollResults;
use crate::models::state::AppState;
use crate::models::vote::VoteResult;
use crate::models::vote::{CreatePollRequest, VoteRequest};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub mod poll;
pub mod state;
pub mod vote;

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
    let results = sqlx::query_as::<_, VoteResult>(
        "SELECT
            chosen_option as option,
            COUNT(*) as count,
            COALESCE(
                ROUND(
                    (COUNT(*) * 100.0 / NULLIF((SELECT COUNT(*) FROM votes WHERE poll_id = $1), 0))
                )::integer,
                0
            ) as percentage
         FROM votes
         WHERE poll_id = $1
         GROUP BY chosen_option",
    )
    .bind(poll_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PollResults { poll_id, results }))
}

// Add this handler function
pub async fn add_poll(
    State(state): State<AppState>,
    Json(poll): Json<CreatePollRequest>,
) -> Result<Json<&'static str>, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO polls (question_text, option_a, option_b) VALUES ($1, $2, $3)",
        poll.question_text,
        poll.option_a,
        poll.option_b
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json("Poll created successfully"))
}
