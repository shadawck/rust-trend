use rtrend;
use chrono::prelude::*;
use rtrend::{country::Country, lang::Lang, property::Property, keywords::Keywords, *};


fn main() {

    let country = Country::new("ALL");
    let lang = Lang::new("fr");
    let property = Property::new("web");
    
    let start_date: Date<Utc> = Utc.ymd(2019, 7, 30);
    let end_date: Date<Utc> = Utc.ymd(2020, 7, 30);

    let keywords = Keywords::new(vec!["baguette", "foie gras"]);

    let _google_client = Client::new(keywords, country)
        .with_property(property)
        .with_date(start_date, end_date)
        .with_lang(lang)
        .build();

    Category::new(184);
    
    let _search_interest = SearchInterest::new(_google_client.clone()).get();
    println!("{}", _search_interest);
    let _region_interest = RegionInterest::new(_google_client.clone()).get_for("baguette");
    //println!("{}", _region_interest);
    let _related_topics = RelatedTopics::new(_google_client.clone()).get();
    //println!("{}", _related_topics);
    let _related_queries = RelatedQueries::new(_google_client.clone()).get();
    //println!("{}", _related_queries);
}
