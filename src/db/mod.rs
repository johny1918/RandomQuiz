use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("Failed to connect to database");
    println!("Connected to database");
    Ok(pool)
}