//! Represent a Google Trend Property

use strum_macros::{Display, EnumString};
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
/// let property = Property::Web;
/// ```
#[derive(PartialEq, Display, Debug, EnumString, Clone)]
pub enum Property {
    #[strum(serialize = "")]
    Web,
    
    #[strum(serialize = "images")]
    Images,
    
    #[strum(serialize = "news")]
    News,
    
    #[strum(serialize = "froogle")]
    Froogle,
    
    #[strum(serialize = "youtube")]
    Youtube,
}
