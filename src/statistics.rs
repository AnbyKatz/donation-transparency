use crate::entities::donation;

/// Simply sums all the donation amounts given a query return
pub fn calculate_total_donations(donations: Vec<donation::Model>) -> i64 {
    donations.iter().map(|d| d.amount).sum()
}
