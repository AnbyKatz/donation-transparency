pub mod models;
pub mod queries;

use sqlx::PgPool;
use std::env;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await
}
