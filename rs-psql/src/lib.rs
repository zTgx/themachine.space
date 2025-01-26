use postgres::{Client, NoTls};
use solagent::lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub struct Engine {
    pub client: Client,
}

lazy_static! {
    /// Global instance of the Engine, initialized once and accessed globally.
    pub static ref DB_ENGINE: Arc<Mutex<Engine>> = {
        let engine = Engine::new();
        Arc::new(Mutex::new(engine))
    };
}

impl Engine {
    pub fn new() -> Self {
        let client = Client::connect("host=localhost user=postgres", NoTls)
            .expect("Postgres server is down!");

        Self { client }
    }
}

impl Engine {
    pub async fn query_blacklist(&mut self, data: String) -> bool {
        let sql = "SELECT * FROM blacklist WHERE address = $1";
        let rows = self
            .client
            .query(sql, &[&data])
            .expect("Failed to query blacklist table");
        rows.len() > 0
    }
}
