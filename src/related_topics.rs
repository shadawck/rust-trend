//! Represent Google Trend Related Topics list.
//!
//! Users searching for your keywords also searched for these topics.
//! You can view by the following metrics:
//! - Top - The most popular topics.
//! Scoring is on a relative scale where a value of 100 is the most commonly searched topic and a value of 50 is a topic searched half as often as the most popular term, and so on.
//! - Rising
//! Related topics with the biggest increase in search frequency since the last time period.
//! Results marked "Breakout" had a tremendous increase, probably because these topics are new and had few (if any) prior searches.

use crate::errors::KeywordNotSet;
use crate::request_handler::Query;
use crate::Client;
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct RelatedTopics {
    pub client: Client,
}

impl RelatedTopics {
    /// Create a `RelatedTopics` Instance.
    ///
    /// Returns a `RelatedTopics` instance
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve Topics data for all keywords set within the client.
    ///
    /// Returns a `serde_json::Value`.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Country, Keywords, Client, RelatedTopics};
    /// let keywords = Keywords::new(vec!["Github vs Gitlab"]);
    /// let country = Country::ALL;
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_topics = RelatedTopics::new(client).get();
    ///
    /// println!("{}", related_topics);
    /// ```
    ///
    /// # Panics
    /// Panic if the client have not been built.
    ///
    /// ```should_panic
    /// # use rtrend::{Country, Keywords, Client, RelatedTopics};
    /// let keywords = Keywords::new(vec!["hacker"]);
    /// let country = Country::US;
    ///
    /// // Client not built
    /// let client = Client::new(keywords, country);
    ///
    /// let related_topics = RelatedTopics::new(client).get();
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

    /// Retrieve Topics data for all keywords filtered by Top Topics in descending order
    /// Returns a `serde_json::Value`.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Country, Keywords, Client, RelatedTopics};
    /// let keywords = Keywords::new(vec!["Github vs Gitlab"]);
    /// let country = Country::ALL;
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_topics = RelatedTopics::new(client).top();
    ///
    /// println!("{}", related_topics);
    /// ```
    pub fn top(&self) -> Value {
        self.get()[0].clone()
    }

    /// Retrieve Topics data for all keywords filtered by Rising Topics in descending order
    /// Returns a `serde_json::Value`.
    ///
    /// # Example
    /// Retrieve Topics data for all keywords filtered by Top Topics in descending order
    /// Returns a `serde_json::Value`.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Country, Keywords, Client, RelatedTopics};
    /// let keywords = Keywords::new(vec!["Github vs Gitlab"]);
    /// let country = Country::ALL;
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_topics = RelatedTopics::new(client).rising();
    ///
    /// println!("{}", related_topics);
    /// ```
    pub fn rising(&self) -> Value {
        self.get()[1].clone()
    }

    /// Retrieve Topics data for a specific keywords.
    ///
    /// Retrieve data for a specific keyword set within the client.
    ///
    /// Returns a JSON serde Value (`serde_json::Value`).
    ///
    /// ```rust
    /// # use rtrend::{Country, Keywords, Client, RelatedTopics};
    /// let keywords = Keywords::new(vec!["Github", "Gitlab"]);
    /// let country = Country::ALL;
    /// let client = Client::new(keywords, country).build();
    ///
    /// let related_topics = RelatedTopics::new(client).get_for("Gitlab");
    ///
    /// println!("{}", related_topics);
    /// ```
    /// # Panics
    /// Will panic if input keyword have not been set previously for the client.
    ///
    /// ```should_panic
    /// # use rtrend::{Country, Keywords, Client, RelatedTopics};
    /// let keywords = Keywords::new(vec!["PS4","XBOX","PC"]);
    /// let country = Country::ALL;
    ///
    /// let client = Client::new(keywords, country).build();
    ///
    /// let region_interest = RelatedTopics::new(client).get_for("WII");
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
