pub trait OnChainQuery {
    fn get_chain(&self) -> String;
    fn get_data(&self) -> String;
}
