use std::fmt;

#[derive(Debug, Clone)]
pub enum Endpoint {
    Explore(String),
    WidgetData(String),
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
