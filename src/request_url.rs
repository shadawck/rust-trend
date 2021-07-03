use reqwest::Url;

// Ex : https://trends.google.com/trends/api/
pub struct RequestUrl {
    base_url: String,
    uri: String,
    endpoint: String,
}

impl RequestUrl {
    pub fn new(endpoint: String, uri: String) -> RequestUrl {
        RequestUrl {
            base_url: "https://trends.google.com/trends/api".to_string(),
            endpoint,
            uri,
        }
    }

    pub fn build(&self) -> Url {
        let url = format!(
            "{}{}{}",
            self.base_url,
            self.endpoint.to_owned(),
            self.uri.to_owned()
        );
        Url::parse(&url).unwrap()
    }
}
