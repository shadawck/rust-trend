use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct KeywordNotSet;
impl Display for KeywordNotSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "The keyword is not set with the client !")
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