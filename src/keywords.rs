#[derive(Debug)]
pub struct Keywords {
    pub keywords : Vec<&'static str>
}

impl Keywords {
    pub fn new(keywords : Vec<&'static str>) -> Self {

        Self {
            keywords : check_keywords(keywords)
        }
    }
}

impl From<&'static str > for Keywords {
    fn from(item: &'static str ) -> Self {
        Self {
            keywords : check_keywords(item.split(',').collect())
        }
    }
}

fn check_keywords(keys : Vec<&'static str>) ->  Vec<&'static str> {
    if keys.len() == 0 {
        panic!("At least one keyword is required !")
    }
    if keys.len() > 5 {
        panic!("The maximum is 5 keywords !")
    }
    keys
}