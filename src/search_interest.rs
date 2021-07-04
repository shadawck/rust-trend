use std::usize;

use crate::client::*;
use crate::utils;
use chrono::prelude::*;
use reqwest::Url;
use serde_json::Value;

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct SearchInterest {
    pub end_date: Date<Utc>,   // Default : Today
    pub start_date: Date<Utc>, // Default : Today
    token: String,
    request: String,
    client: Client,
}

impl SearchInterest {
    const MULTILINE_ENDPOINT: &'static str =
        "https://trends.google.com/trends/api/widgetdata/multiline";
    const BAD_CHARACTER: usize = 5;

    pub fn new(client: Client) -> SearchInterest {
        let end_date = Utc::now().date();
        let start_date = Utc::now().with_year(end_date.year() - 1).unwrap().date();

        let widgets: Value = serde_json::from_str(client.response.as_str()).unwrap();

        let request = widgets["widgets"][0]["request"].to_string();
        let token = widgets["widgets"][0]["token"].to_string().replace("\"", "");

        SearchInterest {
            end_date,
            start_date,
            request,
            token,
            client,
        }
    }

    pub fn with_filter(
        client: Client,
        start_date: Date<Utc>,
        end_date: Date<Utc>,
    ) -> SearchInterest {
        let mut search_interest: SearchInterest = Self::new(client);
        search_interest.start_date = start_date;
        search_interest.end_date = end_date;
        search_interest
    }

    pub fn get(&self) {
        let url = Url::parse(Self::MULTILINE_ENDPOINT).unwrap();

        let resp = self
            .client
            .client_builder
            .get(url)
            .query(&[
                ("hl", self.client.lang),
                ("geo", self.client.country),
                ("tz", "-120"),
                ("req", &self.request),
                ("token", &self.token),
                ("tz", "-120"),
            ])
            .send();
        
        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };

        let body = resp.text().unwrap();
        let response = utils::sanitize_response(&body, Self::BAD_CHARACTER).to_string();
        
        println!("{}", response);
    }
}


