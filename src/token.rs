use reqwest::{blocking::ClientBuilder, header, Url};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub interest_token: String,
    pub region_token : String,
    pub related_topic_token : String,
    pub related_queries_token : String,
}

impl Token {
    const TOKEN_HANDSHAKE: &'static str = "https://trends.google.com/trends/api/explore";

    pub fn retrieve_new_token(cookie: &'static str, keywords: &'static str) -> Token {
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

        let comparison_item = format!("{{'comparisonItem':[{{'keyword':'{}','geo':'FR','time':'today 12-m'}}],'category':0,'property':''}}", keywords);

        let url = Url::parse(Self::TOKEN_HANDSHAKE).unwrap();
        let resp = client_builder
            .get(url)
            .query(&[
                ("hl", "fr"),
                ("tz", "-120"),
                ("req", &comparison_item),
                ("tz", "-120"),
            ])
            .send();

        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get response : {:?}", error),
        };

        let body = resp.text().unwrap();



        let clean_body = clean_resp_to_deserilize(&body, 4);
        let widgets: Value = serde_json::from_str(clean_body).unwrap();


        println!("{}",widgets);

        let interest_token: String = widgets["widgets"][0]["token"].to_string().replace("\"", "");
        let region_token: String = widgets["widgets"][1]["token"].to_string().replace("\"", "");
        let related_topic_token: String = widgets["widgets"][2]["token"].to_string().replace("\"", "");
        let related_queries_token: String = widgets["widgets"][3]["token"].to_string().replace("\"", "");

        Token {
            interest_token,
            region_token,
            related_topic_token,
            related_queries_token
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}



fn clean_resp_to_deserilize(body: &str, pos: usize) -> &str {
    match body.char_indices().skip(pos).next() {
        Some((pos, _)) => &body[pos..],
        None => "",
    }
}

