use rs_strategy::Strategy;

pub struct OnchainEngine {}

impl OnchainEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan(&self, strategy: &Strategy) -> serde_json::Value {
        todo!()
    }
}
