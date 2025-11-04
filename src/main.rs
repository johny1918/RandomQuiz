use axum::{Json, Router};
use axum::routing::get;
use serde_json::json;

mod models;
mod db;
use crate::db::connect_db;
use crate::models::state::AppState;

#[tokio::main]
async fn main() {
    let pool = connect_db().await.unwrap();
    let app_state = AppState::new(pool);
    let app = routes(app_state).await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening on http://127.0.0.1:8000");
    axum::serve(listener, app.into_make_service()).await.unwrap()
}

async fn routes(poll: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .with_state(poll)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
