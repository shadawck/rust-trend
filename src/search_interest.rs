use crate::{client::*, endpoint::*, locale::*, query_builder::*, request_url::*, uri::*};
use chrono::{prelude::*};
use reqwest::{blocking::Response, Error};

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct SearchInterest {
    //RequestBody: String, //Hiden field
    endpoint: Endpoint,
    uri: URI,

    pub locale: Locale,
    pub keywords: String,

    pub start_date: String, // Default : Today
    pub end_date: String,   // Default : Today
}

impl SearchInterest {
    pub fn new(keywords: String, locale: Locale) -> SearchInterest {
        let default_end_date = Utc::now();
        let default_start_date = Utc::now().with_year(default_end_date.year()-1).unwrap();

        SearchInterest {
            keywords,
            locale,
            endpoint: Endpoint::WidgetData("/widgetdata".to_owned()),
            uri: URI::Multiline("/multiline".to_owned()),
            start_date: default_start_date.format("%Y-%m-%d").to_string(),
            end_date: default_end_date.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn with_filter(
        keywords: String,
        locale: Locale,
        start_date: String,
        end_date: String,
    ) -> SearchInterest {
        let mut search_interest: SearchInterest = Self::new(keywords, locale);
        search_interest.start_date = start_date;
        search_interest.end_date = end_date;
        search_interest
    }

    pub fn get(&self, client: Client) {
        let request = self.build_request();


        let url = RequestUrl::new("/widgetdata".to_string(), "/multiline".to_string()).build();
        println!("URL  : {}", url);

        let query = QueryBuilder::build(client.token.interest_token.to_string(), request.to_owned());
        println!("Query : {:?}", query);
        
    }

    fn build_request(&self) -> String {
        let request = format!("{{'time':'{}+{}','resolution':'WEEK','locale':'{}','comparisonItem':[{{'geo':{{'country':'{}'}},'complexKeywordsRestriction':{{'keyword':[{{'type':'BROAD','value':'{}'}}]}}}}],'requestOptions':{{'property':'','backend':'IZG','category':0}}}}", self.start_date, self.end_date, self.locale.to_string().to_lowercase(), self.locale.to_string().to_uppercase(), self.keywords);
        request
    }
}