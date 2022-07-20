//! Represent Google Trend Related Queries list.
//!
//! Users searching for your term also searched for these queries.
//! You can sort by the following metrics:
//! - Top - The most popular search queries.
//! Scoring is on a relative scale where a value of 100 is the most commonly searched query, 50 is a query searched half as often as the most popular query, and so on.
//! - Rising - Queries with the biggest increase in search frequency since the last time period.
//! Results marked "Breakout" had a tremendous increase, probably because these queries are new and had few (if any) prior searches.

use crate::errors::KeywordNotSet;
use crate::request_handler::Query;
use crate::Client;

use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct RelatedQueries {
    pub client: Client,
}

impl RelatedQueries {
    /// Create a `RelatedQueries` Instance.
    ///
    /// Returns a `RelatedQueries` instance
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve Queries data for all keywords.
    ///
    /// Retrieve data for all keywords set within the client.
    ///
    /// Returns a JSON serde Value (`serde_json::Value`).
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Country, Keywords, Client, RelatedQueries};
    /// let keywords = Keywords::new(vec!["Github vs Gitlab"]);
    /// let country = Country::ALL;
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_queries = RelatedQueries::new(client).get();
    ///
    /// println!("{}", related_queries);
    /// ```
    ///
    /// # Panics
    /// Panic if the client have not been built.
    ///
    /// ```should_panic
    /// # use rtrend::{Country, Keywords, Client, RelatedQueries};
    /// let keywords = Keywords::new(vec!["Github vs Gitlab"]);
    /// let country = Country::ALL;
    /// let client = Client::new(keywords, country);
    ///
    /// let related_queries = RelatedQueries::new(client).get();
    /// ```
    pub fn get(&self) -> Value {
        let value = self
            .send_request()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let joined = value.join(",");

        let form: String = format!("[{}]", joined);

        serde_json::from_str(form.as_str()).unwrap()
    }

    /// Retrieve Queries data for a specific keywords.
    ///
    /// Retrieve data for a specific keyword set within the client.
    ///
    /// Returns a JSON serde Value (`serde_json::Value`).
    ///
    /// ```rust
    /// # use rtrend::{Country, Keywords, Client, RelatedQueries};
    /// let keywords = Keywords::new(vec!["Github", "Gitlab"]);
    /// let country = Country::ALL;
    ///
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_queries = RelatedQueries::new(client).get_for("Gitlab");
    ///
    /// println!("{}", related_queries);
    /// ```
    ///
    /// # Panics
    /// Will panic if input keyword have not been set previously for the client.
    ///
    /// ```should_panic
    /// # use rtrend::{Country, Keywords, Client, RelatedQueries};
    /// let keywords = Keywords::new(vec!["PS4","XBOX","PC"]);
    /// let country = Country::ALL;
    ///
    /// let client = Client::new(keywords, country).build();
    ///
    /// let region_interest = RelatedQueries::new(client).get_for("WII");
    /// ```
    pub fn get_for(&self, keyword: &str) -> Value {
        let index = self
            .client
            .keywords
            .keywords
            .iter()
            .position(|&x| x == keyword);
        let keyword_index = match index {
            Some(k) => k,
            None => Err(KeywordNotSet).unwrap(),
        };

        self.send_request()[keyword_index].clone()
    }
}
