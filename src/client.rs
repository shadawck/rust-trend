use crate::utils;
use reqwest::{
    blocking::ClientBuilder,
    blocking::{RequestBuilder, Response},
    header, Error, Url,
};
use serde_json::Value;

#[derive(Default, Debug, Clone)]
pub struct Client {
    pub client_builder: reqwest::blocking::Client,
    cookie: &'static str,
    country: &'static str,
    keywords: &'static str,
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

        let resp = Self::build(client_builder.clone(), lang, country, keywords);
        let response = Self::send(resp);

        Client {
            client_builder,
            country,
            cookie,
            keywords,
            lang,
            response,
        }
    }

    pub fn with_category(mut self, category: u8) -> Client {
        let resp =
            Self::build_with_category(self.client_builder, self.lang, self.country, self.keywords, category);

        let mut client_with_category = Client::new(self.cookie, self.keywords, self.lang, self.country);
        client_with_category.response = Self::send(resp);
        
        client_with_category
    }

    fn build_with_category(
        client_builder: reqwest::blocking::Client,
        lang: &'static str,
        country: &'static str,
        keywords: &'static str,
        category: u8,
    ) -> RequestBuilder {
        let url = Url::parse(Self::EXPLORE_ENDPOINT).unwrap();
        let comparison_item = format!(
            "{{'comparisonItem':[{{
                    'keyword':'{}',
                    'geo':'{}',
                    'time':'today 12-m'
                }}],
                'property':'',
                'backend' : 'IZG',
                'category':'{}',
                
            }}",
            keywords, country, category
        );

        client_builder.get(url).query(&[
            ("hl", lang),
            ("geo", country),
            ("tz", "-120"),
            ("req", &comparison_item),
            ("tz", "-120"),
        ])
    }

    fn build(
        client_builder: reqwest::blocking::Client,
        lang: &'static str,
        country: &'static str,
        keywords: &'static str,
    ) -> RequestBuilder {
        let url = Url::parse(Self::EXPLORE_ENDPOINT).unwrap();
        let comparison_item = format!(
            "{{'comparisonItem':[{{
                    'keyword':'{}',
                    'geo':'{}',
                    'time':'today 12-m'
                }}],
                'category':0,
                'property':''
            }}",
            keywords, country
        );

        client_builder.get(url).query(&[
            ("hl", lang),
            ("geo", country),
            ("tz", "-120"),
            ("req", &comparison_item),
            ("tz", "-120"),
        ])
    }

    fn send(resp: RequestBuilder) -> Value {
        let resp = resp.send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };

        let body = resp.text().unwrap();
        let clean_response = utils::sanitize_response(&body, Self::BAD_CHARACTER).to_string();

        serde_json::from_str(clean_response.as_str()).unwrap()
    }
}
