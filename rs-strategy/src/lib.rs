use rs_primitives::{request::XRequest, traits::OnChainQuery};

#[derive(Debug, Clone)]
pub enum Strategy {
    Default,
    Level0,
    Level1,
    Level2,
}

impl Strategy {
    pub async fn apply(&self, data: &impl OnChainQuery) -> serde_json::Value {
        match self {
            Self::Default => self.apply_default(data).await,
            Self::Level0 => todo!(),
            Self::Level1 => todo!(),
            Self::Level2 => todo!(),
        }
    }

    pub async fn apply_x(&self, data: XRequest) -> serde_json::Value {
        unimplemented!()
    }

    pub async fn apply_default(&self, data: &impl OnChainQuery) -> serde_json::Value {
        // Step 0: Query the database blacklist
        let is_blacklisted = Self::query_blacklist(data.get_chain()).await;

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

    // Example helper functions (placeholders for actual implementations)
    async fn query_blacklist(chain: String) -> bool {
        // Implement the logic to query the blacklist database
        // Return true if found, false otherwise
        true // Placeholder
    }

    async fn get_basic_info(data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to retrieve basic info and detection rate
        serde_json::json!({
            "info": "Basic information",
            "detection_rate": 0.95 // Placeholder
        })
    }

    async fn get_recent_transactions_for_address(data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to get the last three transactions for the address
        serde_json::json!({
            "transactions": vec!["tx1", "tx2", "tx3"] // Placeholder
        })
    }

    async fn get_recent_transactions_for_ca(data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to get the last three transactions for the CA
        serde_json::json!({
            "transactions": vec!["tx1", "tx2", "tx3"] // Placeholder
        })
    }

    async fn get_transaction_info(data: &impl OnChainQuery) -> serde_json::Value {
        // Implement the logic to get the parent transaction info
        serde_json::json!({
            "parent_transaction": "parent_tx",
            "related_transactions": vec!["tx1", "tx2", "tx3"] // Placeholder
        })
    }
}

impl Strategy {
    pub async fn score(data: &serde_json::Value) -> u8 {
        // 90% -> 父交易在黑名单
        // 80% -> parent -> parent 在黑名单
        // 70% -> parent -> parent -> parent 在黑名单

        0 // clean
    }
}
