use rs_primitives::traits::OnChainQuery;
use rs_psql::DB_ENGINE;
use rs_strategy::Strategy;

#[derive(Clone)]
pub struct OnchainEngine {}

impl OnchainEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn scan(&self, strategy: &Strategy, data: &impl OnChainQuery) -> serde_json::Value {
        if strategy.clone() == Strategy::Default {
            return self.apply_default(data).await;
        }

        unimplemented!()
    }
}

impl OnchainEngine {
    pub async fn apply_default(&self, data: &impl OnChainQuery) -> serde_json::Value {
        // Step 0: Query the database blacklist
        let is_blacklisted = Self::query_blacklist(data.get_data()).await;

        if is_blacklisted {
            // If found in blacklist, return basic info and detection rate
            return Self::get_basic_info(data).await;
        }

        // Step 1: If not in the database, check the type of data
        match data.get_data().as_str() {
            "address" => {
                // Step 2.1: Query the last three transactions for the address
                return Self::get_recent_transactions_for_address(data).await;
            }
            "ca" => {
                // Step 2.2: Query the last three transactions for the CA
                return Self::get_recent_transactions_for_ca(data).await;
            }
            "tx" => {
                // Step 2.3: Query the parent transaction and related info
                return Self::get_transaction_info(data).await;
            }
            _ => {
                // Handle unknown data type
                return serde_json::json!({
                    "error": "Unknown data type"
                });
            }
        }
    }

    async fn query_blacklist(data: String) -> bool {
        DB_ENGINE
            .lock()
            .expect("DB_ENGINE")
            .query_blacklist(data)
            .await
    }

    async fn get_basic_info(_data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to retrieve basic info and detection rate
        serde_json::json!({
            "info": "Basic information",
            "detection_rate": 0.95 // Placeholder
        })
    }

    async fn get_recent_transactions_for_address(_data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to get the last three transactions for the address
        serde_json::json!({
            "transactions": vec!["tx1", "tx2", "tx3"] // Placeholder
        })
    }

    async fn get_recent_transactions_for_ca(_data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to get the last three transactions for the CA
        serde_json::json!({
            "transactions": vec!["tx1", "tx2", "tx3"] // Placeholder
        })
    }

    async fn get_transaction_info(_data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to get the parent transaction info
        serde_json::json!({
            "parent_transaction": "parent_tx",
            "related_transactions": vec!["tx1", "tx2", "tx3"] // Placeholder
        })
    }
}
