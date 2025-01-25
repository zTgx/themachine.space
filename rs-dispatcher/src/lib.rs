use rs_primitives::{request::XRequest, traits::OnChainQuery};
use rs_strategy::Strategy;

#[derive(Debug, Clone)]
pub enum DispatcherType {
    Onchain,
    Offchain,
}

#[derive(Debug, Clone)]
pub struct Dispatcher {
    pub r#type: DispatcherType,
    pub strategy: Strategy,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            r#type: DispatcherType::Onchain,
            strategy: Strategy::Default,
        }
    }
}

impl Dispatcher {
    pub async fn dispatch_onchain(&self, data: impl OnChainQuery) -> serde_json::Value {
        self.strategy.apply(&data).await
    }

    pub async fn dispatch_offchain(&self, data: XRequest) -> serde_json::Value {
        self.strategy.apply_x(data).await
    }
}
