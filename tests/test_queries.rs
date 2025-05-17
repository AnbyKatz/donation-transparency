mod tests {
    use sea_orm::{Database, DatabaseConnection};
    use std::env;

    use donation_transparency::queries::*;

    #[tokio::test]
    async fn test_all_financial_years() {
        let db = init_db().await;
        let query = all_financial_years(&db).await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    #[tokio::test]
    async fn test_all_parties() {
        let db = init_db().await;
        let query = all_parties(&db).await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    #[tokio::test]
    async fn test_donations_that_financial_year() {
        let db = init_db().await;
        let query = donations_that_financial_year(&db, "2023-24").await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    #[tokio::test]
    async fn test_search_for_donor() {
        let db = init_db().await;
        let query = search_for_donors(&db, "common%").await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    #[tokio::test]
    async fn test_all_party_donations() {
        let db = init_db().await;
        let years_query = all_financial_years(&db).await.unwrap();
        let query = all_party_donations(&db, 1, &years_query).await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    #[tokio::test]
    async fn test_all_party_branches() {
        let db = init_db().await;
        let query = all_party_branches(&db, 1).await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    #[tokio::test]
    async fn test_all_donor_donations() {
        let db = init_db().await;
        let years_query = all_financial_years(&db).await.unwrap();
        let query = all_donor_donations(&db, 6240, &years_query).await.unwrap();
        assert!(!query.is_empty(), "Expected none empty result");
    }

    async fn init_db() -> DatabaseConnection {
        dotenv::dotenv().ok(); // Load from .env file
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        Database::connect(&db_url)
            .await
            .expect("Failed to connect to database")
    }
}
