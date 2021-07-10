use crate::client::*;
use crate::request_handler::Query;
use serde_json::Value;

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug, Clone)]
pub struct RegionInterest {
    pub client: Client,
}

impl RegionInterest {
    pub fn new(client: Client) -> RegionInterest {
        RegionInterest { client }
    }

    pub fn get(&self) -> Value {
        self.send_request()[0].clone()
    }

    pub fn get_for(&self, keyword: &str) -> Value {
        let index = self
            .client
            .keywords
            .keywords
            .iter()
            .position(|&x| x == keyword);

        let keyword_index = match index {
            Some(k) => k,
            None => panic!("The keyword {} is not set with the client", keyword),
        };

        let response_index = keyword_index + 1;

        self.send_request()[response_index].clone()
    }
}
