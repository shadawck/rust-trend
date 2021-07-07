extern crate rtrend;

use chrono::prelude::*;
use rtrend::{country::Country, lang::Lang, property::Property, keywords::Keywords, *};

fn main() {
    let my_cookie = "NID=218=WdklPXCK9nRe7pPNLmdGZ4w_2cfOsRdeD_ESaCAeDB_0MqJrCqDYOll55y_quoQ8I3RQjQmssffaOcQNj3iMhqK3Wep8DOdcJW93-ZxN6Y87Zg5Z_0Ifw7_X4tVYkvdkJoFL9-QfStcvqlGH5wPiBe0ApgskqQllrqtxGK4mEXk";
    let keywords = "baguette";

    let country = Country::new("ALL");
    let lang = Lang::new("fr");
    let property = Property::new("web");
    
    let start_date: Date<Utc> = Utc.ymd(2019, 7, 30);
    let end_date: Date<Utc> = Utc.ymd(2020, 7, 30);

    let keys = Keywords::new(vec!["baguette", "fromage"]);
    let keys2 = Keywords::from("baguette,fromage");

    let google_client = Client::new(my_cookie, keywords, lang, country)
        .with_property(property)
        .with_date(start_date, end_date)
        .build();
    
    let _search_interest = SearchInterest::new(google_client.clone()).get();
    println!("{}", _search_interest.unwrap());
    let _region_interest = RegionInterest::new(google_client.clone()).get().unwrap();
    println!("{}", _region_interest);
    let _related_topics = RelatedTopics::new(google_client.clone()).get().unwrap();
    println!("{}", _related_topics);
    let _related_queries = RelatedQueries::new(google_client.clone()).get().unwrap();
    println!("{}", _related_queries);
}
