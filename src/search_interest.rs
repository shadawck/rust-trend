use crate::client::Client;
use crate::request_handler::Query;

use serde_json::Value;

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug, Clone)]
pub struct SearchInterest {
    pub client: Client,
}

impl SearchInterest {
    pub fn new(client: Client) -> SearchInterest {
        SearchInterest { client }
    }

    pub fn get(&self) -> Value {
        self.send_request()[0].clone()
    }
}
