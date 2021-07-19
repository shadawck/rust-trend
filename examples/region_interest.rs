use rtrend::{Client, Country, Keywords, RegionInterest};

fn main() {
    let country = Country::new("US");
    let keywords = Keywords::new(vec!["Instagram", "Facebook"]);
    
    let client = Client::new(keywords, country).build();
    
    let region_interest = RegionInterest::new(client).get();
    println!("{}", region_interest);
}
