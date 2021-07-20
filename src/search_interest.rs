//! Represent keywords interest over time. 
//! 
//! Numbers represent search interest relative to the highest point on the chart for the given region and time. 
//! A value of 100 is the peak popularity for the term. A value of 50 means that the term is half as popular.
//! A score of 0 means there was not enough data for this term.

use crate::Client;
use crate::request_handler::Query;

use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct SearchInterest {
    pub client: Client,
}

impl SearchInterest {
    /// Create a SearchInterest instance.
    /// 
    /// Returns a SearchInterest instance
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve line chart data (Timeseries data) for all keywords
    ///
    /// Retrieve data for all keywords set within the client.
    ///
    /// Returns a JSON serde Value (`serde_json::Value`).
    /// ```
    /// # use rtrend::{Country, Keywords, Client, SearchInterest};
    /// let keywords = Keywords::new(vec!["Candy"]);
    /// let country = Country::new("US");
    /// 
    /// let client = Client::new(keywords, country).build();
    /// 
    /// let search_interest = SearchInterest::new(client).get();
    /// 
    /// println!("{}", search_interest);
    /// ```
    pub fn get(&self) -> Value {
        self.send_request()[0].clone()
    }
}
