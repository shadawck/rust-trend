use std::collections::HashMap;

use crate::{
    utils, Client, Keywords, RegionInterest, RelatedQueries, RelatedTopics, SearchInterest,
};
use reqwest::{blocking::RequestBuilder, Url};
use serde_json::Value;

pub trait Query {
    // Build queries for all type of search
    fn build_request(&self) -> Vec<RequestBuilder>;

    // Send queries for request build previously
    fn send_request(&self) -> Vec<Value> {
        const BAD_CHARACTER: usize = 5;
        let mut responses: Vec<Value> = Vec::new();

        for request in self.build_request() {
            let resp = request.send();
            let resp = match resp {
                Ok(resp) => resp,
                Err(error) => panic!("Can't get client response: {:?}", error),
            };
            let body = resp.text().unwrap();
            let clean_response = utils::sanitize_response(&body, BAD_CHARACTER);
            responses.push(serde_json::from_str(clean_response).unwrap())
        }
        responses
    }
}

impl Query for SearchInterest {
    fn build_request(&self) -> Vec<RequestBuilder> {
        const MULTILINE_ENDPOINT: &str =
            "https://trends.google.com/trends/api/widgetdata/multiline";
        let url = Url::parse(MULTILINE_ENDPOINT).unwrap();

        let request = self.client.response["widgets"][0]["request"].to_string();
        let token = self.client.response["widgets"][0]["token"]
            .to_string()
            .replace("\"", "");

        vec![build_query(&self.client, url, request, token)]
    }
}

impl Query for RegionInterest {
    fn build_request(&self) -> Vec<RequestBuilder> {
        const COMPAREDGEO_ENDPOINT: &str =
            "https://trends.google.com/trends/api/widgetdata/comparedgeo";
        let url = Url::parse(COMPAREDGEO_ENDPOINT).unwrap();
        let keywords_nb = self.client.keywords.keywords.len();
        let mut requests: Vec<RequestBuilder> = Vec::new();

        if keywords_nb == 1 {
            let request = self.client.response["widgets"][1]["request"].clone();
            let mod_region_request = mod_region_request(request, self.resolution).to_string();

            let token = self.client.response["widgets"][1]["token"]
                .to_string()
                .replace("\"", "");

            vec![build_query(&self.client, url, mod_region_request, token)]
        } else {
            for i in 1..=keywords_nb {
                let request = self.client.response["widgets"][i * 3]["request"].clone();
                let mod_region_request = mod_region_request(request, self.resolution).to_string();

                println!("{}", mod_region_request);

                let token = self.client.response["widgets"][i * 3]["token"]
                    .to_string()
                    .replace("\"", "");
                requests.push(build_query(
                    &self.client,
                    url.clone(),
                    mod_region_request,
                    token,
                ));
            }

            requests
        }
    }
}

impl Query for RelatedTopics {
    fn build_request(&self) -> Vec<RequestBuilder> {
        const RELATED_SEARCH_ENDPOINT: &str =
            "https://trends.google.com/trends/api/widgetdata/relatedsearches";
        let url = Url::parse(RELATED_SEARCH_ENDPOINT).unwrap();
        let keywords = self.client.keywords.keywords.clone();
        let mut requests: Vec<RequestBuilder> = Vec::new();

        if keywords.len() == 1 {
            let request = self.client.response["widgets"][2]["request"].to_string();
            let token = self.client.response["widgets"][2]["token"]
                .to_string()
                .replace("\"", "");
            vec![build_query(&self.client, url, request, token)]
        } else {
            for keyword in &keywords {
                let individual_keyword = Keywords::new(vec![keyword]);

                let new_client = self
                    .client
                    .clone()
                    .with_keywords(individual_keyword)
                    .build();
                let request = new_client.response["widgets"][2]["request"].to_string();
                let token = new_client.response["widgets"][2]["token"]
                    .to_string()
                    .replace("\"", "");
                requests.push(build_query(&new_client, url.clone(), request, token))
            }

            requests
        }
    }
}

impl Query for RelatedQueries {
    fn build_request(&self) -> Vec<RequestBuilder> {
        const RELATED_QUERY_ENDPOINT: &str =
            "https://trends.google.com/trends/api/widgetdata/relatedsearches";
        let url = Url::parse(RELATED_QUERY_ENDPOINT).unwrap();

        let mut requests: Vec<RequestBuilder> = Vec::new();
        let keywords_nb = self.client.keywords.keywords.len();

        if keywords_nb == 1 {
            let request = self.client.response["widgets"][3]["request"].to_string();
            println!("{}", request);

            let token = self.client.response["widgets"][3]["token"]
                .to_string()
                .replace("\"", "");
            vec![build_query(&self.client, url, request, token)]
        } else {
            for i in 1..=keywords_nb {
                let request = self.client.response["widgets"][i * 3 + 1]["request"].to_string();
                let token = self.client.response["widgets"][i * 3 + 1]["token"]
                    .to_string()
                    .replace("\"", "");
                requests.push(build_query(&self.client, url.clone(), request, token));
            }
            requests
        }
    }
}

fn build_query(client: &Client, url: Url, request: String, token: String) -> RequestBuilder {
    client.client.get(url).query(&[
        ("hl", client.lang.to_string().as_str()),
        ("tz", "-120"),
        ("req", request.as_str()),
        ("token", token.as_str()),
        ("tz", "-120"),
    ])
}

fn mod_region_request(request: Value, resolution: &str) -> Value {
    let mut config: HashMap<String, Value> =
        serde_json::from_value(request).expect("unable to parse JSON request");
    if let Some(mut res) = config["resolution"].as_str() {
        res = resolution;
        config.insert("resolution".to_string(), Value::from(res));
    } else {
        panic!("Unknown resolution");
    }
    serde_json::to_value(config).unwrap()
}
