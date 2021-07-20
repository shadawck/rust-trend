//! Represent Google Trend Country.   
//! 
//! All Countries available [here ](https://github.com/shadawck/rust-trend/wiki/Countries)

use std::fmt;
use crate::errors::errors::UnsupportedCategoryError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Country {
    country: &'static str,
}

impl Country {
    const SUPPORTED_COUNTRY: &'static [&'static str] = &[
        "ALL", "ID", "FI", "SC", "HT", "CG", "BL", "GF", "TD", "DJ", "TL", "GA", "CI", "YT", "TG",
        "GP", "BJ", "CD", "KM", "ML", "MQ", "GN", "SN", "NC", "RE", "CM", "PF", "GG", "GB", "NE",
        "GI", "BI", "FR", "MG", "BF", "MU", "HN", "JE", "KY", "TN", "MR", "DZ", "MA", "IM", "CU",
        "LU", "BE", "QA", "CN", "MW", "SH", "AE", "PE", "SV", "EC", "MX", "BO", "BN", "NI", "BM",
        "CO", "LB", "CH", "PY", "ES", "CL", "UY", "GT", "CA", "CW", "AR", "PA", "VE", "DO", "KH",
        "CR", "SG", "IE", "MO", "RW", "AD", "HK", "AM", "PH", "MY", "PG", "EE", "TT", "SL", "MN",
        "CY", "PR", "SE", "AU", "AO", "SK", "AZ", "CZ", "AL", "IS", "NZ", "KE", "MZ", "KW", "OM",
        "TR", "BH", "MK", "JM", "US", "MT", "XK", "TW", "BT", "DK", "RO", "NL", "PT", "UZ", "GH",
        "ZW", "DE", "PL", "ME", "KR", "PK", "TZ", "IT", "LA", "IN", "RS", "AT", "ZA", "BR", "RU",
        "ET", "MM", "NO", "HU", "NA", "SI", "LV", "MD", "VN", "LT", "LR", "BA", "UG", "NG", "ZM",
        "BG", "MV", "GE", "HR", "NP", "GR", "UA", "KG", "LY", "LK", "IL", "JO", "BY", "EG", "AF",
        "TH", "BD", "SA", "KZ", "PS", "SD", "JP", "BB", "IQ", "YE", "BS", "IR", "SY", "MS", "GQ",
        "ST", "PM", "CF", "GW", "SX", "MP", "KN", "VG", "DM", "TC", "SZ", "VI", "GM", "SR", "BW",
        "GY", "GD", "SO", "FJ", "EH", "AW", "GU", "LC", "SS", "LS", "TM", "TJ", "AI", "AX", "AS",
        "AQ", "TF", "AG", "BQ", "BZ", "BV", "CC", "CK", "CV", "CX", "ER", "FK", "FO", "FM", "GL",
        "HM", "IO", "KI", "LI", "MF", "MC", "MH", "NF", "NU", "NR", "PN", "PW", "KP", "GS", "SJ",
        "SB", "SM", "TK", "TO", "TV", "UM", "VA", "VC", "VU", "WF", "WS",
    ];

    /// Create a new Country.
    /// 
    /// Returns a Country instance.
    /// 
    /// # Example
    /// ```
    /// # use rtrend::Country;
    /// let country = Country::new("FR");
    /// ```
    /// 
    /// # Panics
    /// An unsupported Country id will panic.
    /// ```should_panic
    /// # use rtrend::Country;
    /// let country = Country::new("ZC");
    /// ```
    pub fn new(country: &'static str) -> Country {
        Country {
            country: Self::check_country(country),
        }
    }

    fn check_country(country: &'static str) -> &'static str {
        match Self::SUPPORTED_COUNTRY.contains(&country) {
            true => {
                if country.eq("ALL") {
                    ""
                } else {
                    country
                }
            }
            false => Err(UnsupportedCategoryError).unwrap(),
        }
    }

    /// List supported countries
    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_COUNTRY);
    }

    /// Convert country to &str
    pub fn as_str(&self) -> &'static str {
        self.country
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.country)
    }
}
