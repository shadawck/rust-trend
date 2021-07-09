use core::panic;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Property {
    property: &'static str,
}

impl Property {
    const SUPPORTED_PROPERTY: &'static [&'static str] =
        &["web", "images", "news", "froogle", "youtube"];

    pub fn new(property: &'static str) -> Property {
        Property {
            property: Self::check_property(property),
        }
    }

    fn check_property(property: &'static str) -> &'static str {
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

    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_PROPERTY);
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.property)
    }
}
