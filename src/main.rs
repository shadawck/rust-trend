mod client;
mod endpoint;
mod locale;
mod query_builder;
mod request_url;
mod search_interest;
mod token;
mod uri;

use crate::client::*;
use crate::locale::*;
use crate::search_interest::*;

//struct RegionInterest {}
//struct RelatedTopics {}
//struct RelatedQueries {}

fn main() {
    let my_cookie = "NID=218=WdklPXCK9nRe7pPNLmdGZ4w_2cfOsRdeD_ESaCAeDB_0MqJrCqDYOll55y_quoQ8I3RQjQmssffaOcQNj3iMhqK3Wep8DOdcJW93-ZxN6Y87Zg5Z_0Ifw7_X4tVYkvdkJoFL9-QfStcvqlGH5wPiBe0ApgskqQllrqtxGK4mEXk";
    
    let keywords = "test";
    // Client need Keyword to be set because token is based on it
    //let client = Client::new(my_cookie, keywords); 
    
    let build = Client::build(my_cookie, keywords);

    //let search_result = simple_search.get(client);
}
