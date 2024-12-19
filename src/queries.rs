use log::info;
use sqlx::PgPool;

use crate::models::Donation;

/* Tables to query from */
static DONATION_TABLE: &str = "donation";
static DONAR_TABLE: &str = "donar";
static PARTY_TABLE: &str = "party";
static BRANCH_TABLE: &str = "branch";

/* Query helpers */
fn all_parties_query() -> String {
    format!("SELECT name FROM {}", PARTY_TABLE)
}

fn all_donars_query() -> String {
    format!("SELECT name FROM {}", DONAR_TABLE)
}

fn all_financial_years_query() -> String {
    format!("SELECT DISTINCT year FROM {}", DONATION_TABLE)
}

fn all_party_branches_query() -> String {
    format!("SELECT id FROM {} WHERE party_id = $1", BRANCH_TABLE)
}

fn all_donations_for_party_query() -> String {
    format!(
        r#"SELECT 
            d.id,
            d.year,
            d.amount,
            b.name AS branch_name,
            donar.name AS donar_name
        FROM {} d      
        JOIN Branch b ON d.branch_id = b.id
        JOIN Donar donar ON d.donar_id = donar.id
        WHERE d.branch_id = ANY($1)"#,
        DONATION_TABLE,
    )
}

/* Inbetween queries */
pub async fn get_all_branch_ids(pool: &PgPool, party_id: i32) -> Result<Vec<i32>, sqlx::Error> {
    let query = all_party_branches_query();
    info!("Running query: {query}");
    let result = sqlx::query_scalar::<_, i32>(&query)
        .bind(party_id)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

/// Grabs all party names, these are the top level parents that have
/// multiple branches and is generally used in order to select a specific
/// party to see their aggregate donations
pub async fn get_all_parties(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query = all_parties_query();
    info!("Running query: {query}");
    let parties: Vec<String> = sqlx::query_scalar::<_, String>(&query)
        .fetch_all(pool)
        .await?;
    Ok(parties)
}

/// Grabs all the donars names, used if you want to see how much a specific
/// donar donated and to which parties
pub async fn get_all_donars(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query = all_donars_query();
    info!("Running query: {query}");
    let donars: Vec<String> = sqlx::query_scalar::<_, String>(&query)
        .fetch_all(pool)
        .await?;
    Ok(donars)
}

/// Used to get an aggregate list of financial years for future data filtering
pub async fn get_all_financial_years(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query = all_financial_years_query();
    info!("Running query: {query}");
    let financial_years: Vec<String> = sqlx::query_scalar::<_, String>(&query)
        .fetch_all(pool)
        .await?;
    Ok(financial_years)
}

/// Get all donations for a specific party and all its branch parties
/// This is the main function the project is based on and most data derives
/// from this call
pub async fn get_all_donations_for_party(
    pool: &PgPool,
    party_id: i32,
    year: Option<String>,
) -> Result<Vec<Donation>, sqlx::Error> {
    /* Get all the branch ids for a specific party and if a year exists
    add that as a where clause */
    let branch_ids = get_all_branch_ids(pool, party_id).await?;
    let mut query = all_donations_for_party_query();
    match year {
        Some(x) => {
            query += format!(r#" AND year = '{x}'"#).as_str();
        }
        None => (),
    }

    info!("{query}");
    let donations = sqlx::query_as::<_, Donation>(&query)
        .bind(&branch_ids)
        .fetch_all(pool)
        .await?;
    Ok(donations)
}
