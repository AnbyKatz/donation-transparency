use helpers::{DonationCollection, unique_parties_donated_to};
use sea_orm::{DbConn, DbErr};

pub use helpers::{
    all_financial_years, all_parties, donations_that_financial_year, search_for_donors,
};

use crate::entities::branch;

/// All donations for a single party for a collection of financial years
pub async fn all_party_donations(
    db: &DbConn,
    party_id: i32,
    financial_years: &Vec<String>,
) -> Result<Vec<(String, i64)>, DbErr> {
    let party = helpers::party_by_id(db, party_id).await?;
    let branches = helpers::party_related_branches(db, party).await?;
    let branches_ids = helpers::branches_to_branch_ids(branches).await?;
    let donations = helpers::donations_for_branches(db, branches_ids, financial_years).await?;
    Ok(donations)
}

/// For a given party, get their associated branches via the list of ids
pub async fn all_party_branches(db: &DbConn, party_id: i32) -> Result<Vec<branch::Model>, DbErr> {
    let party = helpers::party_by_id(db, party_id).await?;
    let branches = helpers::party_related_branches(db, party).await?;
    Ok(branches)
}

pub async fn all_donor_donations(
    db: &DbConn,
    donor_id: i32,
    financial_years: &Vec<String>,
) -> Result<helpers::DonationCollection, DbErr> {
    let mut out: DonationCollection = DonationCollection::new();
    let unique_parties = unique_parties_donated_to(db, donor_id, &financial_years).await?;
    for branch_id in unique_parties {
        let donor_contributions =
            helpers::donations_by_donor(db, donor_id, branch_id, financial_years).await?;
        out.insert(donor_contributions.0, donor_contributions.1);
    }
    Ok(out)
}

/// Helper methods un-accessible outside this module
mod helpers {
    use std::collections::HashMap;

    use sea_orm::prelude::Expr;
    use sea_orm::sea_query::extension::postgres::PgExpr;
    use sea_orm::{
        ColumnTrait, DbConn, DbErr, EntityTrait, FromQueryResult, JoinType, ModelTrait, Order,
        QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    };
    use serde::{Deserialize, Serialize};

    use crate::entities::prelude::Branch;
    use crate::entities::{branch, donation, donor, party};
    use crate::statistics;

    // Adapter structs
    /// Used for plotting the data
    pub type DonationCollection = HashMap<String, Vec<YearlyDonation>>;

    #[derive(Serialize)]
    pub struct YearlyDonation {
        pub year: String,
        pub amount: i64,
    }

    /// This is essentially the reconstructed reciept row you would find in the
    /// spreadsheet
    #[derive(Serialize, Deserialize, Clone, Debug, FromQueryResult)]
    pub struct DonationReciept {
        pub party_name: String,
        pub branch_name: String,
        pub donor_name: String,
        pub financial_year: String,
        pub amount: i64,
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

    pub async fn party_by_id(db: &DbConn, id: i32) -> Result<party::Model, DbErr> {
        let selected_party = party::Entity::find_by_id(id).one(db).await?;
        match selected_party {
            Some(x) => return Ok(x),
            None => {
                return Err(DbErr::RecordNotFound(format!(
                    "Party with id {} not found",
                    id
                )));
            }
        }
    }

    pub async fn party_by_branch_id(db: &DbConn, id: i32) -> Result<party::Model, DbErr> {
        let selected_branch = branch::Entity::find_by_id(id).one(db).await?;
        match selected_branch {
            Some(x) => party_by_id(db, x.party_id).await,
            None => {
                return Err(DbErr::RecordNotFound(format!(
                    "Branch with id {} not found",
                    id
                )));
            }
        }
    }

    pub async fn donor_by_id(db: &DbConn, id: i32) -> Result<donor::Model, DbErr> {
        let selected_donor = donor::Entity::find_by_id(id).one(db).await?;
        match selected_donor {
            Some(x) => return Ok(x),
            None => {
                return Err(DbErr::RecordNotFound(format!(
                    "Donor with id {} not found",
                    id
                )));
            }
        }
    }

    /// Get a party's associated branches
    pub async fn party_related_branches(
        db: &DbConn,
        single_party: party::Model,
    ) -> Result<Vec<branch::Model>, DbErr> {
        single_party.find_related(Branch).all(db).await
    }

    /// Get all the branch ids associated with this list of branches
    pub async fn branches_to_branch_ids(branches: Vec<branch::Model>) -> Result<Vec<i32>, DbErr> {
        let branch_ids: Vec<i32> = branches.into_iter().map(|b| b.id).collect();
        Ok(branch_ids)
    }

    /// Get the unique donations a donor made to parties that financial year
    /// i.e. we just want to know if they donated to labor, liberal or
    /// greens, clive palmer etc. (ridiculous examples i know lol)
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `donor_id` - Unique donor i.e. "Common Wealth Bank"'s id
    /// * `financial_year` - e.g. 2023-24
    ///
    /// # Returns
    ///
    /// Vec of the unique donations, we only generally care about the donor ids
    pub async fn unique_parties_donated_to(
        db: &DbConn,
        donor_id: i32,
        financial_years: &Vec<String>,
    ) -> Result<Vec<i32>, DbErr> {
        let unique_parties_donated_to: Vec<i32> = donation::Entity::find()
            .distinct()
            .select_only()
            .column(donation::Column::BranchId)
            .left_join(branch::Entity)
            .left_join(donor::Entity)
            .filter(donation::Column::Year.is_in(financial_years))
            .filter(donor::Column::Id.eq(donor_id))
            .into_tuple()
            .all(db)
            .await?;
        Ok(unique_parties_donated_to)
    }

    /// All donations for a single financial year
    pub async fn donations_that_financial_year(
        db: &DbConn,
        financial_year: &str,
    ) -> Result<Vec<DonationReciept>, DbErr> {
        let results = donation::Entity::find()
            .select_only()
            .column_as(party::Column::Name, "party_name")
            .column_as(branch::Column::Name, "branch_name")
            .column_as(donor::Column::Name, "donor_name")
            .column_as(donation::Column::Year, "financial_year")
            .column(donation::Column::Amount)
            .join(JoinType::InnerJoin, donation::Relation::Branch.def())
            .join(JoinType::InnerJoin, branch::Relation::Party.def())
            .join(JoinType::InnerJoin, donation::Relation::Donor.def())
            .filter(donation::Column::Year.eq(financial_year))
            .into_model::<DonationReciept>()
            .all(db)
            .await?;
        Ok(results)
    }

    pub async fn donations_by_donor(
        db: &DbConn,
        donor_id: i32,
        branch_id: i32,
        financial_years: &Vec<String>,
    ) -> Result<(String, Vec<YearlyDonation>), DbErr> {
        let mut out: Vec<YearlyDonation> = Vec::new();
        for year in financial_years {
            let results = donation::Entity::find()
                .filter(donation::Column::Year.eq(year))
                .filter(donation::Column::BranchId.eq(branch_id))
                .filter(donation::Column::DonorId.eq(donor_id))
                .all(db)
                .await?;
            let total = statistics::calculate_total_donations(results);
            out.push(YearlyDonation {
                year: year.to_owned(),
                amount: total,
            });
        }

        let party = party_by_branch_id(db, branch_id).await?;
        Ok((party.name.to_owned(), out))
    }

    /// Given an array of branch ids, corresponding to one party, and a
    /// single financial year. Find all the donations associated.
    ///
    /// # Arguments
    ///
    /// * `db` - Database connection
    /// * `branches` - All the queried branches, usually obtained from running
    /// `donation-transparency/src/queries::all_parties_branches`
    /// * `financial_years` - e.g. ["2023-24", "2022-23"]
    ///
    /// # Returns
    ///
    /// Key => financial_year, Val => Donations
    pub async fn donations_for_branches(
        db: &DbConn,
        branches_ids: Vec<i32>,
        financial_years: &Vec<String>,
    ) -> Result<Vec<(String, i64)>, DbErr> {
        let mut donations_by_year: Vec<(String, i64)> = Vec::new();
        for year in financial_years {
            let donations = donation::Entity::find()
                .filter(donation::Column::BranchId.is_in(branches_ids.clone()))
                .filter(donation::Column::Year.eq(year.clone()))
                .all(db)
                .await?;
            let total_amount = statistics::calculate_total_donations(donations);
            donations_by_year.push((year.to_owned(), total_amount));
        }
        Ok(donations_by_year)
    }

    /// Case insensitive substring search for a matching donor
    /// There's too many so generally you want to search first for a list
    /// of donors and then select a single one before querying for their donations
    pub async fn search_for_donors(
        db: &DbConn,
        search_string: &str,
    ) -> Result<Vec<donor::Model>, DbErr> {
        donor::Entity::find()
            .filter(Expr::col(donor::Column::Name).ilike(search_string))
            .all(db)
            .await
    }
}
