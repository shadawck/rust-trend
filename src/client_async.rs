use crate::{
    cookie::Cookie, country::Country, keywords::Keywords, lang::Lang, property::Property, utils,
    Category,
};
use chrono::{Date, Utc};
use reqwest::{header, ClientBuilder, Url};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ClientAsync {
    pub client_builder: reqwest::Client,
    pub cookie: Cookie,
    pub country: Country,
    pub keywords: Keywords,
    pub lang: Lang,
    pub property: Property,
    pub time: String,
    pub category: Category,
    pub response: Value,
}

impl Default for ClientAsync {
    fn default() -> Self {
        Self {
            client_builder: Default::default(),
            cookie: Cookie::new(),
            response: serde_json::from_str("{}").unwrap(),
            keywords: Default::default(),
            time: "today 12-m".to_string(),
            country: Country::new("ALL"),
            property: Property::new("web"),
            lang: Lang::new("en"),
            category: Category::new(0),
        }
    }
}

impl ClientAsync {
    const EXPLORE_ENDPOINT: &'static str = "https://trends.google.com/trends/api/explore";
    const BAD_CHARACTER: usize = 4;

    pub fn new(keywords: Keywords, lang: Lang, country: Country) -> Self {
        let mut headers = header::HeaderMap::new();
        headers = Cookie::new().add_to_header(headers);
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
            keywords,
            lang,
            ..Default::default()
        }
    }

    pub fn with_keywords(mut self, keywords: Keywords) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn with_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub fn with_property(mut self, property: Property) -> Self {
        self.property = property;
        self
    }

    pub fn with_period(mut self, period: String) -> Self {
        self.time = period;
        self
    }

    pub fn with_date(mut self, start_date: Date<Utc>, end_date: Date<Utc>) -> Self {
        fn convert(date: Date<Utc>) -> String {
            date.format("%Y-%m-%d").to_string()
        }

        let custom_period = format!("{} {}", convert(start_date), convert(end_date));
        self.time = custom_period;
        self
    }

    pub fn with_filter(mut self, category: Category, property: Property, time: String) -> Self {
        self.category = category;
        self.property = property;
        self.time = time;
        self
    }

    pub async fn build(mut self) -> Self {
        let url = Url::parse(Self::EXPLORE_ENDPOINT).unwrap();
        let comparison_item = self.build_comparison_item();

        let resp = self
            .client_builder
            .get(url)
            .query(&[
                ("hl", self.lang.as_str()),
                ("geo", self.country.as_str()),
                ("tz", "-120"),
                ("req", &comparison_item),
                ("tz", "-120"),
            ])
            .send().await;

        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };

        let body = resp.text().await.unwrap();
        let clean_response = utils::sanitize_response(&body, Self::BAD_CHARACTER).to_string();

        self.response = serde_json::from_str(clean_response.as_str()).unwrap();
        self
    }

    fn build_comparison_item(&self) -> String {
        let mut comparison_item = String::new();
        let keys_it = self.keywords.keywords.iter();

        for key in keys_it {
            let index_value = format!(
                "{{
                    'keyword':'{}',
                    'geo':'{}',
                    'time':'{}'
                }},",
                key, self.country, self.time
            );

            comparison_item.push_str(&index_value);
        }

        format!(
            "{{ 'comparisonItem': [{}], 'category':{}, 'property':'{}' }}",
            comparison_item.as_str(),
            self.category,
            self.property
        )
    }
}
