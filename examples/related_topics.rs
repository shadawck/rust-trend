use rtrend::{Client, Country, Keywords, RelatedTopics};

fn main() {
    let keywords = Keywords::new(vec!["Pasta"]);
    let country = Country::new("IT");

    let client = Client::new(keywords, country).build();

    let search_interest = RelatedTopics::new(client).get();
    println!("{}", search_interest);
}
