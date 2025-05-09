use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::{branch, donation};

/// Get all the branch ids associated with this party
pub async fn get_branch_ids(branches: Vec<branch::Model>) -> Result<Vec<i32>, DbErr> {
    let branch_ids: Vec<i32> = branches.into_iter().map(|b| b.id).collect();
    Ok(branch_ids)
}

/// Given an array of branch ids, corresponding to one party, and a
/// single financial year. Find all the donations associated.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `branches` - All the queried branches, usually obtained from running
/// `donation-transparency/src/queries::all_parties_branchs`
/// * `financial_years` - e.g. ["2023-24", "2022-23"]
///
/// # Returns
///
/// Key => financial_year, Val => Donations
pub async fn get_donations_for_branches(
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
