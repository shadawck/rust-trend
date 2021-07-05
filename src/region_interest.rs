use crate::client::*;
use crate::request_handler::Query;
use serde_json::{Result, Value};

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct RegionInterest {
    pub client: Client,
}

impl RegionInterest {
    pub fn new(client: Client) -> RegionInterest {

        RegionInterest {
            client,
        }
    }

    pub fn get(&self) -> Result<Value>{
        self.send_request()
    }
}
