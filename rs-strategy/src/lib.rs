#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Strategy {
    Default,
    Level0,
    Level1,
    Level2,
}

impl Strategy {
    pub async fn score(_data: &serde_json::Value) -> u8 {
        // 90% -> 父交易在黑名单
        // 80% -> parent -> parent 在黑名单
        // 70% -> parent -> parent -> parent 在黑名单

        0 // clean
    }
}
