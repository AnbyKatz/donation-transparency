mod tests {
    use sea_orm::{Database, DatabaseConnection};
    use std::env;

    use donation_transparency::queries::*;

    const MILLION: i64 = 1000000;

    #[tokio::test]
    async fn test_get_all_parties_returns_some() {
        let db = init_db().await;

        let parties = all_parties(&db).await.unwrap();

        assert!(!parties.is_empty(), "Expected some parties in the database");
    }

    #[tokio::test]
    async fn test_get_all_donations_for_party() {
        let idx = 19;

        let db = init_db().await;
        let parties = all_parties(&db).await.unwrap();
        let party_id = parties[idx].id;
        let donations = all_donations(&db, party_id).await.unwrap();

        println!("Party {} has the following donations:", parties[idx].name);
        for (year, total) in donations {
            println!("{}: ${} million", year, total / MILLION);
        }
    }

    #[tokio::test]
    async fn test_get_all_donations_for_donar() {
        let idx = 19;
        let financial_year = "2022-23";

        let db = init_db().await;
        let parties = all_parties(&db).await.unwrap();
        let party_id = parties[idx].id;
        let donations =
            donations_grouped_by_donar_for_year(&db, party_id, financial_year.to_string())
                .await
                .unwrap();

        println!(
            "Party {} has the following donations for year {}:",
            parties[idx].name, financial_year
        );
        for (donar, total) in donations {
            println!("{}: ${}", donar, total);
        }
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
