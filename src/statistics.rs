use crate::entities::donation;

pub fn calculate_total_donations(donations: Vec<donation::Model>) -> i64 {
    donations.iter().map(|d| d.amount).sum()
}
