use sea_orm::{
    ColumnTrait, DbConn, DbErr, EntityTrait, FromQueryResult, ModelTrait, Order, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::prelude::Branch;
use crate::entities::{branch, donation, donor, party};
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
    let donations = get_donations_for_branches(db, branchs, financial_years).await?;
    let total_donations_by_year: Vec<(String, i64)> = donations
        .into_iter()
        .map(|(year, donations)| {
            let total = statistics::calculate_total_donations(donations);
            (year, total)
        })
        .collect();
    Ok(total_donations_by_year)
}

pub async fn all_party_donations_grouped_by_donor(
    db: &DbConn,
    party_id: i32,
    financial_year: &String,
) -> Result<Vec<(String, i64)>, DbErr> {
    let Some(party) = party_by_id(db, party_id).await.unwrap() else {
        return Err(DbErr::Custom("No party found".to_string()));
    };
    let branchs = all_parties_branchs(db, party).await?;
    let donations = get_donations_grouped_by_donor(db, branchs, financial_year).await?;
    let total_donations_by_donor: Vec<(String, i64)> = donations
        .into_iter()
        .map(|(donor, donations)| {
            let total = statistics::calculate_total_donations(donations);
            (donor, total)
        })
        .collect();
    Ok(total_donations_by_donor)
}

pub async fn all_donor_donations(
    db: &DbConn,
    donor_id: i32,
    financial_years: &Vec<String>,
) -> Result<Vec<(String, i64)>, DbErr> {
    let Some(donor) = donor_by_id(db, donor_id).await.unwrap() else {
        return Err(DbErr::Custom("No donor found".to_string()));
    };
    let donor_donations = get_donations_for_donor(db, donor, financial_years).await?;
    let donor_donations_by_year: Vec<(String, i64)> = donor_donations
        .into_iter()
        .map(|(year, donations)| {
            let total = statistics::calculate_total_donations(donations);
            (year, total)
        })
        .collect();

    Ok(donor_donations_by_year)
}

pub async fn all_parties(db: &DbConn) -> Result<Vec<party::Model>, DbErr> {
    party::Entity::find().all(db).await
}

pub async fn all_donors(db: &DbConn) -> Result<Vec<donor::Model>, DbErr> {
    donor::Entity::find().all(db).await
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

async fn get_branch_ids(branches: Vec<branch::Model>) -> Result<Vec<i32>, DbErr> {
    let branch_ids: Vec<i32> = branches.into_iter().map(|b| b.id).collect();
    Ok(branch_ids)
}

async fn get_donations_for_branches(
    db: &DbConn,
    branches: Vec<branch::Model>,
    financial_years: &Vec<String>,
) -> Result<HashMap<String, Vec<donation::Model>>, DbErr> {
    let mut donations_by_year: HashMap<String, Vec<donation::Model>> = HashMap::new();
    let branch_ids = get_branch_ids(branches).await?;

    for year in financial_years {
        let donations = donation::Entity::find()
            .filter(donation::Column::BranchId.is_in(branch_ids.clone()))
            .filter(donation::Column::Year.eq(year.clone()))
            .all(db)
            .await?;

        donations_by_year.insert(year.to_owned(), donations);
    }

    Ok(donations_by_year)
}

async fn get_donations_for_donor(
    db: &DbConn,
    donor: donor::Model,
    financial_years: &Vec<String>,
) -> Result<HashMap<String, Vec<donation::Model>>, DbErr> {
    let mut donations_by_donor: HashMap<String, Vec<donation::Model>> = HashMap::new();

    for year in financial_years {
        let donations = donation::Entity::find()
            .filter(donation::Column::DonorId.eq(donor.id))
            .filter(donation::Column::Year.eq(year.clone()))
            .all(db)
            .await?;

        donations_by_donor.insert(year.to_owned(), donations);
    }

    Ok(donations_by_donor)
}

async fn get_donations_grouped_by_donor(
    db: &DbConn,
    branches: Vec<branch::Model>,
    financial_year: &String,
) -> Result<HashMap<String, Vec<donation::Model>>, DbErr> {
    let branch_ids = get_branch_ids(branches).await?;

    let donations_with_donors = donation::Entity::find()
        .filter(donation::Column::Year.eq(financial_year))
        .filter(donation::Column::BranchId.is_in(branch_ids))
        .find_also_related(donor::Entity)
        .all(db)
        .await?;

    let mut grouped: HashMap<String, Vec<donation::Model>> = HashMap::new();

    for (donation, maybe_donor) in donations_with_donors {
        if let Some(donor) = maybe_donor {
            grouped.entry(donor.name).or_default().push(donation);
        }
    }

    Ok(grouped)
}
