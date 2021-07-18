# rust-trend

**This lib is a work in progress**

- [x] Write documentation  & README
- [ ] Add "TOP" and "RISING" filter 
- [ ] Write tests
- [ ] Make async feature (currently using Reqwest::blocking)
- [ ] Release on crates.io

## Overview


## Documentation
- [Examples Repository]("./examples")
- [API Documentation]()

## Example

First, add the dependency to your `cargo.toml`:
```toml
[dependencies]
rtrend = "0.1.0"
```

Then build a client and send the reqwest you want : 
```rust
use rtrend::{Keyword, Country, Client, RegionInterest};

let country = Country::new("US");
let keywords = Keywords::new(vec!["Instagram","Facebook"]);
let client = Client::new(keywords, country).build();

// Then select the data you want. The interest of your keywords filtered by region for example:
let region_interest = RegionInterest::new(client).get();
println!("{}", region_interest);

// Result :
//{
//  "default": {
//    "geoMapData": [
//      {
//        "formattedValue": [
//          "100"
//        ],
//        "geoCode": "US-CA",
//        "geoName": "California",
//        "hasData": [
//          true
//        ],
//        "maxValueIndex": 0,
//        "value": [
//          100
//        ]
//      },
//
//      ...
//      
//      {
//        "formattedValue": [
//          "46"
//        ],
//        "geoCode": "US-SD",
//        "geoName": "South Dakota",
//        "hasData": [
//          true
//        ],
//        "maxValueIndex": 0,
//        "value": [
//          46
//        ]
//      }
//    ]
//  }
//}

```

### More example
- [Basic setup]()
- [Region Interest]()
- [Search Interest]()
- [Related Queries]()
- [Related Topics]()
- [Use filters]()
- [Get response for one keywords]()

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.