
/// Lib mod
pub mod client;
pub mod search_interest;
pub mod region_interest;

pub mod related_queries;
pub mod related_topics;

mod request_handler;
mod utils;

/// Path shortcut 
pub use client::Client;
pub use search_interest::SearchInterest;
pub use region_interest::RegionInterest;

pub use related_queries::RelatedQueries;
pub use related_topics::RelatedTopics;


