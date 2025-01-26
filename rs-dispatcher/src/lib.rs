use rs_onchain::OnchainEngine;
use rs_primitives::{request::XRequest, traits::OnChainQuery};
use rs_strategy::Strategy;

#[derive(Debug, Clone)]
pub enum DispatcherType {
    Onchain,
    Offchain,
}

#[derive(Clone)]
pub struct Dispatcher {
    pub r#type: DispatcherType,
    pub strategy: Strategy,
    pub engine: OnchainEngine,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            r#type: DispatcherType::Onchain,
            strategy: Strategy::Default,
            engine: OnchainEngine::new(),
        }
    }
}

impl Dispatcher {
    pub async fn dispatch_onchain(&self, data: impl OnChainQuery) -> serde_json::Value {
        self.engine.scan(&self.strategy, &data).await
    }

    pub async fn dispatch_offchain(&self, _data: XRequest) -> serde_json::Value {
        unimplemented!()
    }
}
