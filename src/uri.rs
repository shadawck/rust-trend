use std::fmt;

#[derive(Debug)]
pub enum URI {
    Multiline(String),
    ComparedGeo(String),
    RelatedSearches(String),
}

impl fmt::Display for URI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
