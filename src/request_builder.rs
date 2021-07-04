use reqwest::{Url, blocking::RequestBuilder};

use crate::{client::Client, search_interest::SearchInterest};

pub trait Query {
    fn build_request(&self) -> RequestBuilder;
}

impl Query for SearchInterest {
    fn build_request(&self)  -> RequestBuilder {
        build_query(self.client.clone(), self.url.clone(), self.request.to_string(), self.token.clone())
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
