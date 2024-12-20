use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Branch {
    pub id: i32,
    pub name: String,
    pub party_id: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Donar {
    pub id: i32,
    pub name: String,
    pub industry_id: Option<i32>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Industry {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Party {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow, Deserialize)]
/// This is NOT an exact copy of the table as
/// this table is almost always used with a join
/// to reconstruct the names
pub struct Donation {
    pub id: i32,
    pub year: String,
    pub amount: i64,
    pub branch_name: String,
    pub donar_name: String,
}
