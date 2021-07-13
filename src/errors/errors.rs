use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct UnsupportedCategoryError;

impl Display for UnsupportedCategoryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported category !")
    }
}

#[derive(Debug)]
pub struct UnsupportedCountryError;
impl Display for UnsupportedCountryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported country !")
    }
}

#[derive(Debug)]
pub struct UnsupportedLangError;
impl Display for UnsupportedLangError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported country !")
    }
}

#[derive(Debug)]
pub struct UnsupportedPropertyError;
impl Display for UnsupportedPropertyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Unsupported property !")
    }
}

#[derive(Debug)]
pub struct KeywordNotSetError {
    keyword: &'static str,
}
impl Display for KeywordNotSetError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "The keyword {} is not set with the client !",
            self.keyword
        )
    }
}

#[derive(Debug)]
pub struct KeywordMaxCapacityError;
impl Display for KeywordMaxCapacityError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "The maximum is 5 keywords !")
    }
}

#[derive(Debug)]
pub struct KeywordMinCapacityError;
impl Display for KeywordMinCapacityError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "At least one keyword is required !")
    }
}

#[allow(dead_code)]
pub enum KeywordsError {
    KeywordMaxCapacityError,
    KeywordMinCapacityError,
}
