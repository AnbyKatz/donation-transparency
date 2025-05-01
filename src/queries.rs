use sea_orm::{
    DbConn, DbErr, EntityTrait, FromQueryResult, ModelTrait, Order, QueryOrder, QuerySelect,
    Statement,
};
use serde::{Deserialize, Serialize};

use crate::entities::prelude::Branch;
use crate::entities::{branch, donation, donor, party};
use crate::helpers;
use crate::statistics;

#[derive(Serialize, Deserialize, Clone, Debug, FromQueryResult)]
pub struct DonationAdapter {
    pub party_name: String,
    pub branch_name: String,
    pub donor_name: String,
    pub financial_year: String,
    pub amount: i64,
}

pub async fn all_donations(
    db: &DbConn,
    financial_year: &str,
) -> Result<Vec<DonationAdapter>, DbErr> {
    let query_string = format!(
        r#"
            SELECT 
                donation.year AS financial_year,
                donation.amount,
                p.name AS party_name,
                b.name AS branch_name,
                d.name AS donor_name
            FROM donation
            JOIN branch b ON b.id = donation.branch_id
            JOIN donor d ON d.id = donation.donor_id
            JOIN party p ON p.id = b.party_id
            WHERE donation.year = '{}'
        "#,
        financial_year
    );
    let results = DonationAdapter::find_by_statement(sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        query_string,
        [],
    ))
    .all(db)
    .await?;

    Ok(results)
}

pub async fn all_party_donations(
    db: &DbConn,
    party_id: i32,
    financial_years: &Vec<String>,
) -> Result<Vec<(String, i64)>, DbErr> {
    let Some(party) = party_by_id(db, party_id).await.unwrap() else {
        return Err(DbErr::Custom("No party found".to_string()));
    };
    let branchs = all_parties_branchs(db, party).await?;
    let donations = helpers::get_donations_for_branches(db, branchs, financial_years).await?;
    let total_donations_by_year: Vec<(String, i64)> = donations
        .into_iter()
        .map(|(year, donations)| {
            let total = statistics::calculate_total_donations(donations);
            (year, total)
        })
        .collect();
    Ok(total_donations_by_year)
}

pub async fn all_parties(db: &DbConn) -> Result<Vec<party::Model>, DbErr> {
    party::Entity::find().all(db).await
}

pub async fn all_financial_years(db: &DbConn) -> Result<Vec<String>, DbErr> {
    donation::Entity::find()
        .select_only()
        .column(donation::Column::Year)
        .distinct()
        .order_by(donation::Column::Year, Order::Asc)
        .into_tuple::<String>()
        .all(db)
        .await
}

pub async fn all_parties_branchs(
    db: &DbConn,
    single_party: party::Model,
) -> Result<Vec<branch::Model>, DbErr> {
    single_party.find_related(Branch).all(db).await
}

pub async fn party_by_id(db: &DbConn, id: i32) -> Result<Option<party::Model>, DbErr> {
    party::Entity::find_by_id(id).one(db).await
}

pub async fn donor_by_id(db: &DbConn, id: i32) -> Result<Option<donor::Model>, DbErr> {
    donor::Entity::find_by_id(id).one(db).await
}

pub async fn search_for_donors(
    db: &DbConn,
    search_string: &str,
) -> Result<Vec<donor::Model>, DbErr> {
    donor::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"SELECT * FROM donor WHERE name ILIKE '%' || $1 || '%'"#,
            [search_string.into()],
        ))
        .all(db)
        .await
}
