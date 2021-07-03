use crate::token::*;
use reqwest::{blocking::ClientBuilder, header, Url};
use serde_json::Value;
use std::fmt;

pub struct Client {
    pub cookie: &'static str,
    pub keywords: &'static str,
    pub token: Token,
}

impl Client {
    const TOKEN_HANDSHAKE: &'static str = "https://trends.google.com/trends/api/explore";

    pub fn new(cookie: &'static str, keywords: &'static str) -> Client {
        Client {
            cookie,
            keywords,
            token: Token::retrieve_new_token(cookie, keywords),
        }
    }

    pub fn build(cookie: &'static str, keywords: &'static str) {
        let mut headers = header::HeaderMap::new();
        headers.insert("Cookie", header::HeaderValue::from_static(cookie));
        let client_builder = ClientBuilder::new().default_headers(headers).build();
        let client_builder = match client_builder {
            Ok(client_builder) => client_builder,
            Err(error) => panic!(
                "Problem constructing the client while retrieving access token: {:?}",
                error
            ),
        };

        let url = Url::parse(Self::TOKEN_HANDSHAKE).unwrap();

        let comparison_item = format!("{{'comparisonItem':[{{'keyword':'{}','geo':'FR','time':'today 12-m'}}],'category':0,'property':''}}", keywords);

        let resp = client_builder
            .get(url)
            .query(&[
                ("hl", "fr"),
                ("tz", "-120"),
                ("req", &comparison_item),
                ("tz", "-120"),
            ]).send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get response : {:?}", error),
        };

        let body = resp.text().unwrap();

        let clean_body = clean_resp_to_deserilize(&body, 4);
        let widgets: Value = serde_json::from_str(clean_body).unwrap();
        let search_interest_request = widgets["widgets"][0]["request"].to_string();
        let search_interest_token = widgets["widgets"][0]["token"].to_string().replace("\"", "");

        let resp_interest = client_builder
            .get("https://trends.google.com/trends/api/widgetdata/multiline".to_owned())
            .query(&[
                ("hl", "fr"),
                ("tz", "-120"),
                ("req", &search_interest_request),
                ("token", &search_interest_token ),
                ("tz", "-120"),
            ]).send();

            let resp_interest= match resp_interest {
                Ok(resp_interest) => resp_interest,
                Err(error) => panic!("Can't get response on Search Interest endpoint : {:?}", error),
            };

            let search_interest_body = resp_interest.text().unwrap();

            let clean_body_interest = clean_resp_to_deserilize(&search_interest_body, 5);

            let search_interest_response: Value = serde_json::from_str(clean_body_interest).unwrap();

            println!("{}", search_interest_response);
    }
}

fn clean_resp_to_deserilize(body: &str, pos: usize) -> &str {
    match body.char_indices().skip(pos).next() {
        Some((pos, _)) => &body[pos..],
        None => "",
    }
}



