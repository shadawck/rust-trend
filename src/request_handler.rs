use crate::region_interest::RegionInterest;
use crate::related_queries::RelatedQueries;
use crate::related_topics::RelatedTopics;

use crate::utils;
use reqwest::{blocking::RequestBuilder};
use reqwest::Url;
use serde_json::{Result, Value};

use crate::{client::Client, search_interest::SearchInterest};

pub trait Query {
    // Build queries for all type of search
    fn build_request(&self) -> RequestBuilder;
    // Send queries for request build previously
    fn send_request(&self) -> Result<Value> {
        const BAD_CHARACTER: usize = 5;

        let resp = self.build_request().send();
        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };
        let body = resp.text().unwrap();
        let clean_response = utils::sanitize_response(&body, BAD_CHARACTER);
        serde_json::from_str(clean_response)
    }
}

impl Query for SearchInterest {
    fn build_request(&self) -> RequestBuilder {
        const MULTILINE_ENDPOINT: &'static str = "https://trends.google.com/trends/api/widgetdata/multiline";
        let url = Url::parse(MULTILINE_ENDPOINT).unwrap();

        let request = self.client.response["widgets"][0]["request"].to_string();
        let token = self.client.response["widgets"][0]["token"]
            .to_string()
            .replace("\"", "");

        build_query(self.client.clone(), url, request, token)
    }
}

impl Query for RegionInterest {
    fn build_request(&self) -> RequestBuilder {
        const COMPAREDGEO_ENDPOINT: &'static str = "https://trends.google.com/trends/api/widgetdata/comparedgeo";
        let url = Url::parse(COMPAREDGEO_ENDPOINT).unwrap();

        let request = self.client.response["widgets"][1]["request"].to_string();
        let token = self.client.response["widgets"][1]["token"]
            .to_string()
            .replace("\"", "");

        build_query(self.client.clone(), url, request, token)
    }
}

impl Query for RelatedTopics {
    fn build_request(&self) -> RequestBuilder {
        const RELATED_SEARCH_ENDPOINT: &'static str = "https://trends.google.com/trends/api/widgetdata/relatedsearches";
        let url = Url::parse(RELATED_SEARCH_ENDPOINT).unwrap();

        let request = self.client.response["widgets"][2]["request"].to_string();
        let token = self.client.response["widgets"][2]["token"]
            .to_string()
            .replace("\"", "");

        build_query(self.client.clone(), url, request, token)
    }
}

impl Query for RelatedQueries {
    fn build_request(&self) -> RequestBuilder {
        const RELATED_QUERY_ENDPOINT: &'static str = "https://trends.google.com/trends/api/widgetdata/relatedsearches";
        let url = Url::parse(RELATED_QUERY_ENDPOINT).unwrap();

        let request = self.client.response["widgets"][3]["request"].to_string();
        let token = self.client.response["widgets"][3]["token"]
            .to_string()
            .replace("\"", "");

        build_query(self.client.clone(), url, request, token)
    }
}


fn build_query(client: Client, url: Url, request: String, token: String) -> RequestBuilder {
    client.client_builder.get(url).query(&[
        ("hl", client.lang),
        ("tz", "-120"),
        ("req", request.as_str()),
        ("token", token.as_str()),
        ("tz", "-120"),
    ])
}
