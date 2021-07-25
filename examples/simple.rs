use rtrend::{Client, Country, Keywords, RegionInterest};

fn main(){
    let country = Country::US;
    let keywords = Keywords::new(vec!["Instagram","Facebook"]);
    
    let client = Client::new(keywords, country).build();
    
    // Then select the data you want. The interest of your keywords filtered by region for example:
    let region_interest = RegionInterest::new(client).get();
    println!("{}", region_interest);
}