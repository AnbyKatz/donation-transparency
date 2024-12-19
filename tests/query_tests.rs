#[cfg(test)]
mod tests {
    use donation_transparency::init_db;
    use donation_transparency::queries::{
        get_all_branch_ids, get_all_donars, get_all_donations_for_party, get_all_financial_years,
        get_all_parties,
    };

    #[tokio::test]
    async fn test_get_all_parties() {
        let pool = init_db().await.unwrap();
        let parties = get_all_parties(&pool).await.unwrap();
        assert!(parties.len() > 0); // Assert that parties were returned
        assert!(parties.contains(&"Labor".to_string())); // Example check
    }

    #[tokio::test]
    async fn test_get_all_donars() {
        let pool = init_db().await.unwrap();
        let donars = get_all_donars(&pool).await.unwrap();
        assert!(donars.len() > 0); // Assert that donars were returned
        assert!(donars.contains(&"Mark Bailey".to_string())); // Example check
    }

    #[tokio::test]
    async fn test_get_all_financial_years() {
        let pool = init_db().await.unwrap();
        let years = get_all_financial_years(&pool).await.unwrap();
        assert!(years.len() > 0); // Assert that years were returned
        assert!(years.contains(&"2022-23".to_string())); // Example check
    }

    #[tokio::test]
    async fn test_get_all_branch_ids() {
        let pool = init_db().await.unwrap();
        let ids = get_all_branch_ids(&pool, 1).await.unwrap();
        assert!(ids.len() > 0); // Assert that years were returned
    }

    #[tokio::test]
    async fn test_get_all_donations_for_party() {
        let pool = init_db().await.unwrap();
        let donations = get_all_donations_for_party(&pool, 1, Some("2022-23".to_string()))
            .await
            .unwrap();
        assert!(donations.len() > 0); // Assert that donations were returned
        assert_eq!(donations[0].year, "2022-23".to_string()); // Check if the year matches
        assert!(donations[0].amount > 0); // Check if amount is valid    
    }
}
