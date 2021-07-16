use crate::{client::*, request_handler::Query};
use serde_json::Value;

/// Represent Google Trend Related Queries list.
/// 
/// Users searching for your term also searched for these queries.
/// You can sort by the following metrics:
/// - <p>* <b>Top</b> - The most popular search queries.
/// Scoring is on a relative scale where a value of 100 is the most commonly searched query, 50 is a query searched half as often as the most popular query, and so on.
/// - <p>* <b>Rising</b> - Queries with the biggest increase in search frequency since the last time period.
/// Results marked "Breakout" had a tremendous increase, probably because these queries are new and had few (if any) prior searches.
#[derive(Clone, Debug, Default)]
pub struct RelatedQueries {
    pub client: Client,
}

impl RelatedQueries {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve Queries data for all keywords.
    ///
    /// Retrieve data for all keywords set within the client.
    ///
    /// Returns a JSON serde Value (serde_json::Value).
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Country, Keywords, Client, RelatedQueries};
    /// let keywords = Keywords::new(vec!["Github vs Gitlab"]);
    /// let country = Country::new("ALL");
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
    /// let country = Country::new("ALL");
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
    /// Returns a JSON serde Value (serde_json::Value).
    ///
    /// ```rust
    /// # use rtrend::{Country, Keywords, Client, RelatedQueries};
    /// let keywords = Keywords::new(vec!["Github", "Gitlab"]);
    /// let country = Country::new("ALL");
    ///
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_queries = RelatedQueries::new(client).get_for("Gitlab");
    ///
    /// println!("{}", related_queries);
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
            None => panic!("The keyword {} is not set with the client", keyword),
        };

        self.send_request()[keyword_index].clone()
    }
}
