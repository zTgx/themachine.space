use super::{chain::Chain, traits::OnChainQuery};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddressRequest {
    pub chain: Chain,
    pub address: String,
}

impl OnChainQuery for AddressRequest {
    fn get_chain(&self) -> String {
        self.chain.to_string()
    }

    fn get_data(&self) -> String {
        self.address.clone()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CaRequest {
    pub chain: Chain,
    pub ca: String,
}

impl OnChainQuery for CaRequest {
    fn get_chain(&self) -> String {
        self.chain.to_string()
    }

    fn get_data(&self) -> String {
        self.ca.clone()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxRequest {
    pub chain: Chain,
    pub tx_hash: String,
}

impl OnChainQuery for TxRequest {
    fn get_chain(&self) -> String {
        self.chain.to_string()
    }

    fn get_data(&self) -> String {
        self.tx_hash.clone()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XRequest {
    pub link: String,
}
