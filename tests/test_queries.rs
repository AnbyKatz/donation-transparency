mod tests {
    use sea_orm::{Database, DatabaseConnection};
    use std::env;

    use donation_transparency::queries::*;

    #[tokio::test]
    async fn test_get_all_parties_returns_some() {
        let db = init_db().await;
        let parties = all_parties(&db).await.unwrap();
        assert!(!parties.is_empty(), "Expected some parties in the database");
    }

    #[tokio::test]
    async fn test_all_donations() {
        let db = init_db().await;
        let donations = all_donations(&db, "2022-23").await.unwrap();
        assert!(
            !donations.is_empty(),
            "Expected some donations in the database"
        );
    }

    #[tokio::test]
    async fn test_get_donation_by_id() {
        let db = init_db().await;
        let donation = donor_by_id(&db, 1).await.unwrap();
        assert!(
            donation.is_some(),
            "Expected a donation with id 1 to exist in the database"
        );
    }

    #[tokio::test]
    async fn test_all_parties_branchs() {
        let db = init_db().await;
        let party = party_by_id(&db, 1).await.unwrap().unwrap();
        let branches = all_parties_branchs(&db, party).await.unwrap();
        assert!(
            !branches.is_empty(),
            "Expected some branches for party with id 1 in the database"
        );
    }

    #[tokio::test]
    async fn test_get_donor_donations() {
        let db = init_db().await;
        let donor_donations = all_donor_donations_for_financial_year(&db, 12610, "2023-24")
            .await
            .unwrap();
        assert!(!donor_donations.is_empty(), "Missing donations for donor");
    }

    async fn init_db() -> DatabaseConnection {
        dotenv::dotenv().ok(); // Load from .env file
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        Database::connect(&db_url)
            .await
            .expect("Failed to connect to database")
    }
}
