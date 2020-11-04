extern crate regex;
use regex::Regex;
use std::str;

static regex_swift: &str = "^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$";

pub(crate) struct Lib{
    
}

impl Lib {


    pub fn check_swift(swift: String) -> bool{
        let reg_ex = Regex::new(regex_swift).unwrap();
        reg_ex.is_match(&swift)
    }

}