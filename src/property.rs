use core::panic;
use std::fmt;

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
    pub fn new(property: &'static str) -> Property {
        Property {
            property: Self::check_property(property),
        }
    }

    fn check_property(property: &'static str) -> &'static str{
        match Self::SUPPORTED_PROPERTY.contains(&property) {
            true => {
                if property.eq("web") {
                    ""
                } else {
                    property
                }
            }
            false => panic!("Unsupported property !"),
        }
    }

    /// List supported properties
    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_PROPERTY);
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.property)
    }
}
