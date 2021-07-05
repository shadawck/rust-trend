use crate::utils;
use reqwest::{Url, blocking::{ClientBuilder, RequestBuilder}, header};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_builder: reqwest::blocking::Client,
    cookie: &'static str,
    pub country: &'static str,
    pub keywords: &'static str,
    pub lang: &'static str,
    pub response: Value,
}

impl Client {
    const EXPLORE_ENDPOINT: &'static str = "https://trends.google.com/trends/api/explore";
    const BAD_CHARACTER: usize = 4;

    pub fn new(
        cookie: &'static str,
        keywords: &'static str,
        lang: &'static str,
        country: &'static str,
    ) -> Client {
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

        let url = Url::parse(Self::EXPLORE_ENDPOINT).unwrap();

        let comparison_item = format!("{{'comparisonItem':[{{'keyword':'{}','geo':'{}','time':'today 12-m'}}],'category':0,'property':''}}", keywords, country);

        let resp = client_builder
            .get(url)
            .query(&[
                ("hl", lang),
                ("geo", country),
                ("tz", "-120"),
                ("req", &comparison_item),
                ("tz", "-120"),
            ])
            .send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };

        let body = resp.text().unwrap();
        let clean_response = utils::sanitize_response(&body, Self::BAD_CHARACTER).to_string();

        let response: Value = serde_json::from_str(clean_response.as_str()).unwrap();

        Client {
            client_builder,
            country,
            cookie,
            keywords,
            lang,
            response,
        }
    }
}

