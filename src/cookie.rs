use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cookie {
    pub nid: String,
}

impl Cookie {
    pub fn new() -> Self {
        Self {
            nid: Self::get_new_cookie(),
        }
    }

    pub fn get_new_cookie() -> String {
        const COOKIE_HANDSHAKE: &str = "https://consent.google.com/";

        let response = reqwest::blocking::get(COOKIE_HANDSHAKE).unwrap();
        let cookie = response.headers().get(SET_COOKIE).unwrap();

        cookie
            .to_str()
            .unwrap()
            .to_string()
            .split(' ')
            .collect::<Vec<&str>>()[0]
            .to_string()
    }

    pub fn add_to_header(&self, mut header: HeaderMap) -> HeaderMap {
        header.insert("Cookie", HeaderValue::from_str(self.nid.as_str()).unwrap());
        header
    }
}
