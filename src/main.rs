use crate::models::{get_random_poll, get_results};
use crate::models::submit_vote;
use axum::Router;
use axum::routing::{get, post};

mod db;
mod models;
use crate::db::connect_db;
use crate::models::health_check;
use crate::models::state::AppState;

#[tokio::main]
async fn main() {
    let pool = connect_db().await.unwrap();
    let app_state = AppState::new(pool);
    let app = routes(app_state).await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:8000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

async fn routes(poll: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/poll", get(get_random_poll))
        .route("/vote", post(submit_vote))
        .route("/results/{poll_id}", get(get_results))
        .with_state(poll)
}
