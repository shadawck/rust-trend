use rtrend::{Client, Country, Keywords, RegionInterest};

fn main() {
    let country = Country::new("US");
    let keywords = Keywords::new(vec!["Instagram", "Facebook", "Pinterest"]);
    
    let client = Client::new(keywords, country).build();
    
    let region_interest_pinterest = RegionInterest::new(client).get_for("Pinterest");
    println!("{}", region_interest_pinterest);
}
