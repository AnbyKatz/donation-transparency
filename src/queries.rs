use sqlx::PgPool;
use std::collections::HashMap;

use crate::models::{Donar, Donation, Party};

/* Tables to query from */
static DONATION_TABLE: &str = "donation";
static DONAR_TABLE: &str = "donar";
static PARTY_TABLE: &str = "party";
static BRANCH_TABLE: &str = "branch";

/* Types */
pub type PartyIdMap = HashMap<String, i32>;

/* Query helpers */
fn all_parties_query() -> String {
    format!("SELECT * FROM {PARTY_TABLE}")
}

fn all_donars_query() -> String {
    format!("SELECT * FROM {DONAR_TABLE}")
}

fn all_financial_years_query() -> String {
    format!("SELECT DISTINCT year as \"year: _\" FROM {DONATION_TABLE}")
}

fn all_party_branches_query() -> String {
    format!("SELECT id FROM {BRANCH_TABLE} WHERE party_id = $1")
}

fn all_donations_for_party_query() -> String {
    format!(
        r#"SELECT 
            d.id,
            d.year,
            d.amount,
            b.name AS branch_name,
            donar.name AS donar_name
        FROM {DONATION_TABLE} d      
        JOIN Branch b ON d.branch_id = b.id
        JOIN Donar donar ON d.donar_id = donar.id
        WHERE d.branch_id = ANY($1)"#,
    )
}

/* Inbetween queries */
pub async fn get_all_branch_ids(pool: &PgPool, party_id: i32) -> Result<Vec<i32>, sqlx::Error> {
    let query = all_party_branches_query();
    let result = sqlx::query_scalar::<_, i32>(&query)
        .bind(party_id)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

/// Grabs all party names, these are the top level parents that have
/// multiple branches and is generally used in order to select a specific
/// party to see their aggregate donations
pub async fn get_all_parties(pool: &PgPool) -> Result<Vec<Party>, sqlx::Error> {
    let query = all_parties_query();
    let parties: Vec<Party> = sqlx::query_as::<_, Party>(&query).fetch_all(pool).await?;
    Ok(parties)
}

/// Grabs all the donars names, used if you want to see how much a specific
/// donar donated and to which parties
pub async fn get_all_donars(pool: &PgPool) -> Result<Vec<Donar>, sqlx::Error> {
    let query = all_donars_query();
    let donars: Vec<Donar> = sqlx::query_as::<_, Donar>(&query).fetch_all(pool).await?;
    Ok(donars)
}

/// Used to get an aggregate list of financial years for future data filtering
pub async fn get_all_financial_years(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let query = all_financial_years_query();
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

    let donations = sqlx::query_as::<_, Donation>(&query)
        .bind(&branch_ids)
        .fetch_all(pool)
        .await?;
    Ok(donations)
}

/// Map to get the internal ID for a party based on its name
/// Generally the name will be given to the user not the ID
/// This should only be called once for the backend generally
pub async fn get_party_id_map(pool: &PgPool) -> Result<PartyIdMap, sqlx::Error> {
    let parties = get_all_parties(pool).await?;
    let party_id_map = parties
        .into_iter()
        .map(|party| (party.name, party.id))
        .collect();
    Ok(party_id_map)
}
