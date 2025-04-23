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
        let donations = all_donations(&db).await.unwrap();
        assert!(
            !donations.is_empty(),
            "Expected some donations in the database"
        );
    }

    #[tokio::test]
    async fn test_get_donation_by_id() {
        let db = init_db().await;
        let donation = donar_by_id(&db, 1).await.unwrap();
        assert!(
            donation.is_some(),
            "Expected a donation with id 1 to exist in the database"
        );
    }

    #[tokio::test]
    async fn test_all_party_donations_grouped_by_donar() {
        let db = init_db().await;
        let donations = all_party_donations_grouped_by_donar(&db, 1, &"2022-23".to_string())
            .await
            .unwrap();
        assert!(
            !donations.is_empty(),
            "Expected some donations in the database"
        );
    }

    #[tokio::test]
    async fn test_all_donar_donations() {
        let db = init_db().await;
        let years = vec!["2022-23".to_string()];
        let donations = all_donar_donations(&db, 1, &years).await.unwrap();
        assert!(
            !donations.is_empty(),
            "Expected some donations in the database"
        );
    }

    #[tokio::test]
    async fn test_all_donars() {
        let db = init_db().await;
        let donars = all_donars(&db).await.unwrap();
        assert!(!donars.is_empty(), "Expected some donars in the database");
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
    async fn test_get_all_financial_years() {
        let db = init_db().await;
        let years = all_financial_years(&db).await.unwrap();
        println!("Found {} financial years", years.len());
    }

    async fn init_db() -> DatabaseConnection {
        dotenv::dotenv().ok(); // Load from .env file
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        Database::connect(&db_url)
            .await
            .expect("Failed to connect to database")
    }
}
