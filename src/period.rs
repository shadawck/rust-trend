//! Represent period predefined by Google Trend.   
//!
//! All period available [here](https://github.com/shadawck/rust-trend/wiki/period)

use strum_macros::{Display, EnumString};

/// Create a predefined Period.
///
/// Returns a Period instance.
///
/// # Example
/// ```
/// # use rtrend::Period;
/// let lang = Period::OneDay;
/// ```
#[derive(Eq, PartialEq, Debug, EnumString, Clone, Display)]
pub enum Period {
    #[strum(serialize = "now 1-H")]
    OneHour,
    #[strum(serialize = "now 4-H")]
    FourHour,

    #[strum(serialize = "now 1-d")]
    OneDay,

    #[strum(serialize = "now 7-d")]
    SevenDay,
    #[strum(serialize = "today 1-m")]
    ThirtyDay,
    #[strum(serialize = "today 3-m")]
    NinetyDay,
    #[strum(serialize = "today 12-m")]
    OneYear,
    #[strum(serialize = "today 5-y")]
    FiveYear,

    #[strum(serialize = "all")]
    Since2004,
}
