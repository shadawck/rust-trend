//! Represent a Google Trend Property

use std::fmt::{Display, Formatter, Result};

use crate::errors::UnsupportedProperty;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Property {
    property: &'static str,
}

impl Property {
    const SUPPORTED_PROPERTY: &'static [&'static str] =
        &["web", "images", "news", "froogle", "youtube"];

    /// Create a new Property.
    ///
    /// The available property are :
    /// - `web`, `images`, `news`, `froogle` (Google Shopping), and `youtube`
    ///
    /// Returns a `Property` instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::Property;
    /// let property = Property::new("news");
    /// ```
    ///
    /// # Panics
    /// An unsupported `Property` will panic.
    /// ```should_panic
    /// # use rtrend::Property;
    /// let property = Property::new("maps");
    /// ```
    pub fn new(property: &'static str) -> Self {
        Self {
            property: Self::check_property(property),
        }
    }

    fn check_property(property: &'static str) -> &'static str {
        if Self::SUPPORTED_PROPERTY.contains(&property) {
            if property.eq("web") {
                ""
            } else {
                property
            }
        } else {
            Err(UnsupportedProperty).unwrap()
        }
    }

    /// List supported properties
    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_PROPERTY);
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.property)
    }
}
