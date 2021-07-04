use std::usize;

use crate::client::*;
use crate::utils;
use reqwest::Url;
use serde_json::{Result, Value};

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct RegionInterest {
    token: String,
    request: Value,
    client: Client,
}

impl RegionInterest {
    const COMPAREDGEO_ENDPOINT: &'static str =
        "https://trends.google.com/trends/api/widgetdata/comparedgeo";
    const BAD_CHARACTER: usize = 5;

    pub fn new(client: Client) -> RegionInterest {

        let widgets: Value = serde_json::from_str(client.response.as_str()).unwrap();

        let request = widgets["widgets"][1]["request"].clone();
        let token = widgets["widgets"][1]["token"].to_string().replace("\"", "");

        RegionInterest {
            request,
            token,
            client,
        }
    }

    pub fn get(&self) -> Result<Value> {
        let url = Url::parse(Self::COMPAREDGEO_ENDPOINT).unwrap();

        let resp = self.client.build_request(
            url.clone(),
            self.request.to_string(),
            self.token.clone(),
        ).send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };

        let body = resp.text().unwrap();
        let clean_response = utils::sanitize_response(&body, Self::BAD_CHARACTER);
        serde_json::from_str(clean_response)
    }
}
