use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenvy::from_filename("../.env").ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL chưa được set trong .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}