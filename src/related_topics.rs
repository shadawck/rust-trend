use crate::client::*;
use crate::request_handler::Query;
use chrono::prelude::*;
use serde_json::Value;

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug, Clone)]
pub struct RelatedTopics {
    pub end_date: Date<Utc>,   // Default : Today
    pub start_date: Date<Utc>, // Default : Today
    pub client: Client,
}

impl RelatedTopics {
    pub fn new(client: Client) -> RelatedTopics {
        let end_date = Utc::now().date();
        let start_date = Utc::now().with_year(end_date.year() - 1).unwrap().date();

        RelatedTopics {
            end_date,
            start_date,
            client,
        }
    }

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
