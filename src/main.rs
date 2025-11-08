use crate::models::submit_vote;
use crate::models::{add_poll, get_random_poll, get_results};
use axum::Router;
use axum::routing::{get, post};
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};


mod db;
mod models;
use crate::db::connect_db;
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
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows all origins (ok for development)
        .allow_methods(Any) // Allows all HTTP methods
        .allow_headers(Any); // Allows all headers

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .expect("Failed to build governor config");
    let governor_limiter = GovernorLayer::new(governor_conf);

    let static_dir = ServeDir::new("static");
    let index_file = ServeFile::new("static/index.html");
    Router::new()
        .route("/poll", get(get_random_poll))
        .route("/results/{poll_id}", get(get_results))
        .route("/vote", post(submit_vote))
        .route("/admin/poll", post(add_poll))
        .route("/admin/delete/{poll_id}", post(models::delete_poll))
        .nest_service("/static", static_dir)
        .fallback_service(index_file)
        .with_state(poll)
        .layer(cors)
        .layer(governor_limiter)
}
