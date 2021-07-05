use core::panic;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Lang {
    property: &'static str,
}

impl Lang {
    const SUPPORTED_PROPERTY: &'static [&'static str] = &["images", "news", "froogle", "youtube"];

    pub fn new(property: &'static str) -> Lang {
        Lang {
            property: Self::check_property(property),
        }
    }

    fn check_property(property: &'static str) -> &'static str {
        match Self::SUPPORTED_PROPERTY.contains(&property) {
            true => property,
            false => panic!("Unsupported property !"),
        }
    }

    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_PROPERTY);
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.property)
    }
}
