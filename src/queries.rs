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

pub async fn fetch_all_financial_years(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query_str = format!("SELECT DISTINCT year FROM {DONATION_TABLE}");
    info!("Running query: {query_str}");
    sqlx::query_scalar::<_, String>(&query_str).fetch_all(pool).await
}

pub async fn fetch_donations_from_party(
    pool: &PgPool,
    party: &str,
    year: Option<String>
) -> Result<Vec<Donation>, sqlx::Error> {
    let party_query = format!(r#"SELECT id FROM {PARTY_TABLE} WHERE name = '{party}'"#);
    info!("{party_query}");
    let party_id = sqlx::query_scalar::<_, i32>(&party_query).fetch_one(pool).await?;

    let mut donation_query = format!(
        r#"SELECT 
            d.id,
            d.year,
            d.amount,
            p.name AS party_name,
            dn.name AS donar_name
        FROM {DONATION_TABLE} d      
        JOIN Party p ON d.party_id = p.id
        JOIN Donar dn ON d.donar_id = dn.id
        WHERE d.party_id = {party_id}          
    "#
    );
    match year {
        Some(x) => {
            info!("{x}");
            donation_query += format!(r#" AND year = '{x}'"#).as_str();
        }
        None => (),
    }
    info!("{donation_query}");
    sqlx::query_as::<_, Donation>(&donation_query).fetch_all(pool).await
}
