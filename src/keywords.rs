use std::fmt::{Display, Formatter, Result};

#[derive(Default, Debug, Clone)]
pub struct Keywords {
    pub keywords : Vec<&'static str>
}

impl Keywords {
    pub fn new(keywords : Vec<&'static str>) -> Self {

        Self {
            keywords : check_keywords(keywords)
        }
    }
}

impl From<&'static str > for Keywords {
    fn from(item: &'static str ) -> Self {
        Self {
            keywords : check_keywords(item.split(',').collect())
        }
    }
}

impl Into<String> for Keywords{
    fn into(self) -> String {
        self.keywords.join(",")
    }
}

fn check_keywords(keys : Vec<&'static str>) ->  Vec<&'static str> {
    if keys.len() == 0 {
        panic!("At least one keyword is required !")
    }
    if keys.len() > 5 {
        panic!("The maximum is 5 keywords !")
    }
    keys
}

impl Display for Keywords {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:#?}", self.keywords)
    }
}