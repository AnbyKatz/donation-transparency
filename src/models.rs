use serde::Serialize;

pub static DONATION_TABLE: &str = "donations";
pub static DONAR_TABLE: &str = "donar";
pub static PARTY_TABLE: &str = "party";

#[derive(Serialize, sqlx::FromRow)]
pub struct Donation {
    pub id: i32,
    pub year: String,
    pub amount: i64,
    pub party_name: String,
    pub donar_name: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Donar {
    pub id: i32,
    pub name: String,
    pub industry: Option<String>,
    pub parent: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Party {
    pub id: i32,
    pub name: String,
    pub parent: Option<String>,
}
