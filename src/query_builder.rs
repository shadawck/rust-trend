use crate::token::*;

pub struct QueryBuilder {
    token: Token,
    mandatory_param: (String, String),
    request: String,
}

impl QueryBuilder {
    pub fn build(token: String, request: String) -> [(String, String); 5] {
        [
            ("hl".to_owned() , "fr".to_owned()),
            ("tz".to_owned(), "-120".to_owned()),
            ("req".to_owned(), request.to_owned()),
            ("token".to_owned(), token.to_owned()),
            ("tz".to_owned(), "-120".to_owned())
        ]
    }
}
