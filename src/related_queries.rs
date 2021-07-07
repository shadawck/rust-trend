use crate::{client::*, request_handler::Query};
use serde_json::{Result, Value};

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct RelatedQueries {
    pub client: Client,
}

impl RelatedQueries {
    pub fn new(client: Client) -> RelatedQueries {
        RelatedQueries {
            client,
        }
    }

    pub fn get(&self) -> Result<Value> {
        self.send_request()
    }
}
