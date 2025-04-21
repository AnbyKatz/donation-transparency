
use sea_orm::{
    ColumnTrait, DbConn, DbErr, EntityTrait, ModelTrait, Order, QueryFilter, QueryOrder,
    QuerySelect,
};
use std::collections::HashMap;

use crate::entities::prelude::Branch;
use crate::entities::{branch, donar, donation, party};
use crate::statistics;

pub async fn all_donations(db: &DbConn, party_id: i32) -> Result<Vec<(String, i64)>, DbErr> {
    let Some(party) = party_by_id(db, party_id).await.unwrap() else {
        return Err(DbErr::Custom("No party found".to_string()));
    };
    let financial_years = all_financial_years(db).await?;
    let branchs = all_parties_branchs(db, party).await?;
    let donations = all_donations_for_branches(db, branchs, financial_years).await?;
    let total_donations_by_year: Vec<(String, i64)> = donations
        .into_iter()
        .map(|(year, donations)| {
            let total = statistics::calculate_total_donations(donations);
            (year, total)
        })
        .collect();
    Ok(total_donations_by_year)
}

pub async fn donations_grouped_by_donar_for_year(
    db: &DbConn,
    party_id: i32,
    financial_year: String,
) -> Result<Vec<(String, i64)>, DbErr> {
    let Some(party) = party_by_id(db, party_id).await.unwrap() else {
        return Err(DbErr::Custom("No party found".to_string()));
    };
    let branchs = all_parties_branchs(db, party).await?;
    let donations = all_donations_grouped_by_donar(db, branchs, financial_year).await?;
    let total_donations_by_donar: Vec<(String, i64)> = donations
        .into_iter()
        .map(|(donar, donations)| {
            let total = statistics::calculate_total_donations(donations);
            (donar, total)
        })
        .collect();
    Ok(total_donations_by_donar)
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

async fn party_by_id(db: &DbConn, id: i32) -> Result<Option<party::Model>, DbErr> {
    party::Entity::find_by_id(id).one(db).await
}

async fn all_parties_branchs(
    db: &DbConn,
    single_party: party::Model,
) -> Result<Vec<branch::Model>, DbErr> {
    single_party.find_related(Branch).all(db).await
}

async fn all_branch_ids(branches: Vec<branch::Model>) -> Result<Vec<i32>, DbErr> {
    let branch_ids: Vec<i32> = branches.into_iter().map(|b| b.id).collect();
    Ok(branch_ids)
}

async fn all_donations_for_branches(
    db: &DbConn,
    branches: Vec<branch::Model>,
    financial_years: Vec<String>,
) -> Result<HashMap<String, Vec<donation::Model>>, DbErr> {
    let mut donations_by_year: HashMap<String, Vec<donation::Model>> = HashMap::new();
    let branch_ids = all_branch_ids(branches).await?;

    for year in financial_years {
        let donations = donation::Entity::find()
            .filter(donation::Column::BranchId.is_in(branch_ids.clone()))
            .filter(donation::Column::Year.eq(year.clone()))
            .all(db)
            .await?;

        donations_by_year.insert(year, donations);
    }

    Ok(donations_by_year)
}

async fn all_donations_grouped_by_donar(
    db: &DbConn,
    branches: Vec<branch::Model>,
    financial_year: String,
) -> Result<HashMap<String, Vec<donation::Model>>, DbErr> {
    let branch_ids = all_branch_ids(branches).await?;

    let donations_with_donars = donation::Entity::find()
        .filter(donation::Column::Year.eq(financial_year))
        .filter(donation::Column::BranchId.is_in(branch_ids))
        .find_also_related(donar::Entity)
        .all(db)
        .await?;

    let mut grouped: HashMap<String, Vec<donation::Model>> = HashMap::new();

    for (donation, maybe_donar) in donations_with_donars {
        if let Some(donar) = maybe_donar {
            grouped.entry(donar.name).or_default().push(donation);
        }
    }

    Ok(grouped)
}
