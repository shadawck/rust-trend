use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct UnsupportedCategory;

impl Display for UnsupportedCategory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported category !")
    }
}

#[derive(Debug)]
pub struct UnsupportedCountry;
impl Display for UnsupportedCountry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported country !")
    }
}

#[derive(Debug)]
pub struct UnsupportedLang;
impl Display for UnsupportedLang {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported country !")
    }
}

#[derive(Debug)]
pub struct UnsupportedProperty;
impl Display for UnsupportedProperty {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported property !")
    }
}

#[derive(Debug)]
pub struct KeywordNotSet {
    keyword: &'static str,
}
impl Display for KeywordNotSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "The keyword {} is not set with the client !",
            self.keyword
        )
    }
}

#[derive(Debug)]
pub struct KeywordMaxCapacity;
impl Display for KeywordMaxCapacity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "The maximum is 5 keywords !")
    }
}

#[derive(Debug)]
pub struct KeywordMinCapacity;
impl Display for KeywordMinCapacity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "At least one keyword is required !")
    }
}