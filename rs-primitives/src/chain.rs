// use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Chain {
    Evm = 0,
    Solana,
    Base,
    Arbitrum,
    Bsc,
    Matic,
    Celo,
    Fantom,
    Avalanche,
    Heco,
    Tron,
    Near,
    Polkadot,
    Kusama,
    Ckb,
    Xdc,
    Eos,
    BinanceSmartChainTestnet,
    EthereumGoerliTestnet,
    OptimismTestnet,
    PolygonMumbaiTestnet,
    SolanaTestnet,
}

impl Chain {
    pub fn to_string(&self) -> String {
        match self {
            Chain::Evm => "EVM".to_string(),
            Chain::Solana => "Solana".to_string(),
            Chain::Base => "Base".to_string(),
            Chain::Arbitrum => "Arbitrum".to_string(),
            Chain::Bsc => "BSC".to_string(),
            Chain::Matic => "Matic".to_string(),
            Chain::Celo => "Celo".to_string(),
            Chain::Fantom => "Fantom".to_string(),
            Chain::Avalanche => "Avalanche".to_string(),
            Chain::Heco => "Heco".to_string(),
            Chain::Tron => "Tron".to_string(),
            Chain::Near => "Near".to_string(),
            Chain::Polkadot => "Polkadot".to_string(),
            Chain::Kusama => "Kusama".to_string(),
            Chain::Ckb => "CKB".to_string(),
            Chain::Xdc => "XDC".to_string(),
            Chain::Eos => "EOS".to_string(),
            Chain::BinanceSmartChainTestnet => "Binance Smart Chain Testnet".to_string(),
            Chain::EthereumGoerliTestnet => "Ethereum Goerli Testnet".to_string(),
            Chain::OptimismTestnet => "Optimism Testnet".to_string(),
            Chain::PolygonMumbaiTestnet => "Polygon Mumbai Testnet".to_string(),
            Chain::SolanaTestnet => "Solana Testnet".to_string(),
        }
    }
}
