use crate::client::Client;
use crate::request_handler::Query;

use chrono::prelude::*;
use serde_json::{Result, Value};

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct SearchInterest {
    pub end_date: Date<Utc>,   // Default : Today
    pub start_date: Date<Utc>, // Default : Today
    pub client: Client,
}

impl SearchInterest {
    pub fn new(client: Client) -> SearchInterest {
        let end_date = Utc::now().date();
        let start_date = Utc::now().with_year(end_date.year() - 1).unwrap().date();

        SearchInterest {
            end_date,
            start_date,
            client,
        }
    }

    pub fn get(&self) -> Result<Value> {
        self.send_request()
    }
}
