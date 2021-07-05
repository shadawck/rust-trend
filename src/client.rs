use crate::utils;
use reqwest::{blocking::ClientBuilder, header, Url};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_builder: reqwest::blocking::Client,
    pub cookie: &'static str,
    pub country: &'static str,
    pub keywords: &'static str,
    pub lang: &'static str,
    pub property: &'static str,
    pub time: &'static str,
    pub category: u8,
    pub response: Value,
}

impl Default for Client {
    fn default() -> Client {
        Client {
            client_builder: Default::default(),
            cookie: Default::default(),
            response: serde_json::from_str("{}").unwrap(),
            keywords: Default::default(),
            time: "today 12-m",
            country: "",
            property: "",
            lang: "en-US",
            category: 0,
        }
    }
}

impl Client {
    const EXPLORE_ENDPOINT: &'static str = "https://trends.google.com/trends/api/explore";
    const BAD_CHARACTER: usize = 4;

    pub fn new(
        cookie: &'static str,
        keywords: &'static str,
        lang: &'static str,
        country: &'static str,
    ) -> Self {
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

        Self {
            client_builder,
            country,
            cookie,
            keywords,
            lang,
            ..Default::default()
        }
    }

    pub fn with_category(mut self, category: u8) -> Self {
        self.category = category;
        self
    }

    pub fn with_property(mut self, property: &'static str) -> Self {
        self.property = property;
        self
    }

    pub fn with_period(mut self, period: &'static str) -> Self {
        self.time = period;
        self
    }

    pub fn with_filter(mut self, category: u8, property: &'static str, time: &'static str) -> Self {
        self.category = category;
        self.property = property;
        self.time = time;
        self
    }

    pub fn build(mut self) -> Self {
        let url = Url::parse(Self::EXPLORE_ENDPOINT).unwrap();
        let comparison_item = format!(
            "{{'comparisonItem':[{{
                    'keyword':'{}',
                    'geo':'{}',
                    'time':'today 12-m'
                }}],
                'category': {},
                'property':'{}'
            }}",
            self.keywords, self.country, self.category, self.property
        );

        let resp = self
            .client_builder
            .get(url)
            .query(&[
                ("hl", self.lang),
                ("geo", self.country),
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

        self.response = serde_json::from_str(clean_response.as_str()).unwrap();
        self
    }
}
