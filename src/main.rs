mod client;
mod endpoint;
mod locale;
mod query_builder;
mod request_url;
mod search_interest;
mod token;
mod uri;
mod utils;

use chrono::prelude::*;

use crate::client::*;
use crate::search_interest::*;

//struct RegionInterest {}
//struct RelatedTopics {}
//struct RelatedQueries {}

fn main() {
    let my_cookie = "NID=218=WdklPXCK9nRe7pPNLmdGZ4w_2cfOsRdeD_ESaCAeDB_0MqJrCqDYOll55y_quoQ8I3RQjQmssffaOcQNj3iMhqK3Wep8DOdcJW93-ZxN6Y87Zg5Z_0Ifw7_X4tVYkvdkJoFL9-QfStcvqlGH5wPiBe0ApgskqQllrqtxGK4mEXk";
    let keywords = "test";
    let country = "FR";
    let lang = "fr";
    let start_date: Date<Utc> = Utc.ymd(2014, 7, 8);
    let end_date: Date<Utc> = Utc.ymd(2020, 12, 20);

    
    let google_client = Client::new(my_cookie, keywords, lang, country);
    
    SearchInterest::new(google_client.clone()).get();
    SearchInterest::with_filter(google_client.clone(), start_date, end_date);


    //let search_result = simple_search.get(client);
}
