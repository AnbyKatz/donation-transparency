use log::info;
use sqlx::PgPool;

use crate::models::{ DONATION_TABLE, PARTY_TABLE, DONAR_TABLE, Donation };

pub async fn fetch_all_parties(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query_str = format!("SELECT name FROM {PARTY_TABLE}");
    info!("Running query: {query_str}");
    sqlx::query_scalar::<_, String>(&query_str).fetch_all(pool).await
}

pub async fn fetch_all_donars(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query_str = format!("SELECT name FROM {DONAR_TABLE}");
    info!("Running query: {query_str}");
    sqlx::query_scalar::<_, String>(&query_str).fetch_all(pool).await
}

pub async fn fetch_donations(pool: &PgPool) -> Result<Vec<Donation>, sqlx::Error> {
    let query_str = format!(
        "SELECT 
            d.id,
            d.year,
            d.amount,
            p.name AS party_name,
            dn.name AS donar_name
        FROM {DONATION_TABLE} d
        JOIN Party p ON d.party_id = p.id
        JOIN Donar dn ON d.donar_id = dn.id"
    );
    info!("Running query: {query_str}");
    sqlx::query_as::<_, Donation>(&query_str).fetch_all(pool).await
}
