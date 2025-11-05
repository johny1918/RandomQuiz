use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use crate::models::poll::Poll;
use crate::models::state::AppState;

pub mod poll;
pub mod vote;
pub mod state;


pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}



pub async fn get_random_poll(
    State(state): State<AppState>
) -> Result<Json<Poll>, (StatusCode, String)> {
    let result = sqlx::query_as!(Poll,
        "SELECT * FROM polls ORDER BY RANDOM() LIMIT 1"
    )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "No polls available".to_string()))?;

    Ok(Json(result))
}