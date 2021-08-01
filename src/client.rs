//! Client used to initialize everything needed by the Google Trend API.

use crate::{utils, Category, Cookie, Country, Keywords, Lang, Period, Property};
use chrono::{Date, Utc};
use reqwest::{blocking::ClientBuilder, header, Url};
use serde_json::Value;
use std::string::ToString;
use strum::EnumProperty;

#[derive(Clone, Debug)]
pub struct Client {
    pub client: reqwest::blocking::Client,
    pub cookie: Cookie,
    pub country: Country,
    pub keywords: Keywords,
    pub lang: Lang,
    pub property: Property,
    pub time: String,
    pub category: Category,
    pub response: Value,
}

/// Default value for client
///
/// Returns a Default Client.  
///
/// By default,
/// - The requested period is 1 year
/// - The Country is all the countries supported by google trend
/// - The Langage is English
/// - The Category is 0
/// - The response is empty (but valid json)
///
/// # Example
/// ```
/// # use rtrend::{Client, Keywords, Country};
/// let keywords = Keywords::new(vec!["rust"]);
/// let country = Country::FR;
///
/// let client = Client::new(keywords, country);
///
/// println!("{:#?}", client);
/// ```
impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::default(),
            cookie: Cookie::new(),
            response: serde_json::from_str("{}").unwrap(),
            keywords: Keywords::default(),
            time: Period::OneYear.to_string(),
            country: Country::ALL,
            property: Property::Web,
            lang: Lang::EN,
            category: Category::All,
        }
    }
}

impl Client {
    const EXPLORE_ENDPOINT: &'static str = "https://trends.google.com/trends/api/explore";
    const BAD_CHARACTER: usize = 4;

    /// Create a new Client.
    ///
    /// Returns a Client.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country};
    /// let keywords = Keywords::new(vec!["rust"]);
    /// let country = Country::FR;
    ///
    /// let client = Client::new(keywords, country);
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if the client can't be built.
    /// This can happen if the cookie can not be set or if the request time out.
    pub fn new(keywords: Keywords, country: Country) -> Self {
        let mut headers = header::HeaderMap::new();
        headers = Cookie::new().add_to_header(headers);
        let client = ClientBuilder::new().default_headers(headers).build();
        let client = match client {
            Ok(client) => client,
            Err(error) => panic!(
                "Problem constructing the client while retrieving access token: {:?}",
                error
            ),
        };

        Self {
            client,
            country,
            keywords,
            ..Client::default()
        }
    }

    /// Set keywords and replace the ones setup during the client creation.
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country};
    /// let keywords = Keywords::new(vec!["rust"]);
    /// let country = Country::FR;
    /// let client = Client::new(keywords, country);
    ///
    /// // ...
    ///
    /// let new_keywords = Keywords::new(vec!["python", "c++"]);
    /// let modified_client = client.with_keywords(new_keywords);
    /// ```
    pub fn with_keywords(mut self, keywords: Keywords) -> Self {
        self.keywords = keywords;
        self
    }
    /// Set in which langage the response will be. The input need to be set in lowercase.
    ///
    /// By default, the response is set to english (en).
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country, Lang};
    /// let keywords = Keywords::new(vec!["rust"]);
    /// let country = Country::ALL;
    /// let lang = Lang::FR;
    ///
    /// // Set response langage to french
    /// let client = Client::new(keywords, country).with_lang(lang);
    /// ```
    pub fn with_lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    /// Set the category google trend will search on.
    ///
    /// By default, any category is set.
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country, Category};
    /// let keywords = Keywords::new(vec!["hacking"]);
    /// let country = Country::ALL;
    /// let category = Category::EngineeringAndTechnology;
    ///
    /// // Set category to "Engineering & Technology"
    /// let client = Client::new(keywords, country).with_category(category);
    /// ```
    pub fn with_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    /// Set the property google trend will search on.
    ///
    /// By default, the search will be made on Google Search (web)
    /// The available property are :
    /// - `web`, `images`, `news`, `froogle` (Google Shopping), `youtube`
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country, Property};
    /// let keywords = Keywords::new(vec!["vlog"]);
    /// let country = Country::ALL;
    ///
    /// // The response will be retrieve from youtube data
    /// let property = Property::Youtube;
    ///
    /// let client = Client::new(keywords, country).with_property(property);
    /// ```
    pub fn with_property(mut self, property: Property) -> Self {
        self.property = property;
        self
    }

    /// Set the period google trend will search on.
    ///
    /// Period are preset set by Google Trend.
    /// By default, the search will be made on 1 year (starting by today).
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country, Period};
    /// let keywords = Keywords::new(vec!["vlog"]);
    /// let country = Country::ALL;
    ///
    /// // response will concern data from this week
    /// let client = Client::new(keywords, country).with_period(Period::SevenDay);
    /// ```
    pub fn with_period(mut self, period: impl ToString) -> Self {
        self.time = period.to_string();
        self
    }

    /// Set the "start date" and "end date" google trend will search on.
    /// By default, the search will be made on 1 year (starting by today).
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country};
    /// # use chrono::prelude::*;
    /// let keywords = Keywords::new(vec!["vlog"]);
    /// let country = Country::ALL;
    ///
    /// // response will concern data from April 25, 2020 to July 30, 2021
    /// let start_date: Date<Utc> = Utc.ymd(2017, 4, 25);
    /// let end_date: Date<Utc> = Utc.ymd(2020, 7, 30);
    ///
    /// let client = Client::new(keywords, country).with_date(start_date, end_date);
    /// ```
    pub fn with_date(mut self, start_date: Date<Utc>, end_date: Date<Utc>) -> Self {
        fn convert(date: Date<Utc>) -> String {
            date.format("%Y-%m-%d").to_string()
        }

        let custom_period = format!("{} {}", convert(start_date), convert(end_date));
        self.time = custom_period;
        self
    }

    /// Allow to set options in one shot.
    ///
    /// For now I don't think it's very useful but if it is, I will make it public
    ///
    /// Returns a client instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country, Property, Category, Lang};
    /// let keywords = Keywords::new(vec!["cat"]);
    /// let country = Country::ALL;
    ///
    /// let client = Client::new(keywords, country).with_filter(
    ///     Category::PetsAndAnimals,
    ///     Property::Images,           // Search on Google Images
    ///     "today 3-m".to_string(),    // 90 previous days
    ///     Lang::IT                    // in italian
    /// );
    /// ```
    #[allow(dead_code)]
    pub fn with_filter(
        mut self,
        category: Category,
        property: Property,
        period: String,
        lang: Lang,
    ) -> Self {
        self.category = category;
        self.property = property;
        self.time = period;
        self.lang = lang;
        self
    }

    /// Build client and send request.
    ///
    /// A response will be retrieve and available through the `response` field.
    /// This field will serve for making next requests.
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Client, Keywords, Country};
    /// let keywords = Keywords::new(vec!["Cat"]);
    /// let country = Country::US;
    ///
    /// let client = Client::new(keywords, country).build();
    ///
    /// println!("{}", client.response);
    /// ```
    pub fn build(mut self) -> Self {
        let url = Url::parse(Self::EXPLORE_ENDPOINT).unwrap();
        let comparison_item = self.build_comparison_item();

        let resp = self
            .client
            .get(url)
            .query(&[
                ("hl", self.lang.to_string().as_str()),
                ("geo", self.country.to_string().as_str()),
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

        let id = self.category.get_int("Id").unwrap_or(0);

        format!(
            "{{ 'comparisonItem': [{}], 'category':{}, 'property':'{}' }}",
            comparison_item.as_str(),
            id,
            self.property
        )
    }
}
