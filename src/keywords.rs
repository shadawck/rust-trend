use std::fmt::{Display, Formatter, Result};

/// A list of keywords to query on Google Trend
/// Keywords is limited to a maximum of 5 keywords.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Keywords {
    pub keywords: Vec<&'static str>,
}

impl Keywords {
    /// Create a new set of keywords.
    ///
    /// Keywords vector is limited to a maximum of 5 keyword.
    ///
    /// Returns a Keywords instance.
    ///
    /// # Example
    ///```rust
    /// use rtrend::Keywords;
    /// let keywords = Keywords::new(vec!["Unicorn","Labradoodle","Pikachu"]);
    /// ```
    ///
    /// # Panics
    /// A vector of length greater than 5 will panic.
    /// ```should_panic
    /// # use rtrend::Keywords;
    /// let seven_dwarf = vec!["Bashful","Doc", "Dopey","Grumpy","Happy", "Sleepy", "Sneezy"];
    /// let keywords = Keywords::new(seven_dwarf);
    /// ```
    ///
    /// A vector without keywords will also panic.
    /// ```should_panic
    /// # use rtrend::Keywords;
    /// let keywords = Keywords::new(vec![]);
    /// ```
    pub fn new(keywords: Vec<&'static str>) -> Self {
        Self {
            keywords: check_keywords(keywords),
        }
    }
}

impl From<&'static str> for Keywords {
    fn from(item: &'static str) -> Self {
        Self {
            keywords: check_keywords(item.split(',').collect()),
        }
    }
}

impl Into<String> for Keywords {
    fn into(self) -> String {
        self.keywords.join(",")
    }
}

fn check_keywords(keys: Vec<&'static str>) -> Vec<&'static str> {
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
