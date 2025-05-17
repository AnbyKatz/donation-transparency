use log::info;
use sea_orm::{Database, DatabaseConnection};
use std::env;

mod statistics;

pub mod entities;
pub mod queries;

/// Get DB connection from .env / environment variable `DATABASE_URL`
pub async fn init_db() -> DatabaseConnection {
    dotenv::dotenv().ok(); // Load from .env file
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    info!("Attempting to connect to {}", db_url);
    Database::connect(&db_url)
        .await
        .expect("Failed to connect to database")
}
