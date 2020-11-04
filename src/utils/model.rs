extern crate regex;
use regex::Regex;
use std::str;

/// Regular expression patter to check a valid SWIFT code
static REGEX_SWIFT: &str = r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$";

/// Regular expression patter to check a valid IBAN code
static REGEX_IBAN: &str = r"([A-Z]{2})\s*\t*(\d\d)\s*\t*(\d\d\d\d)\s*\t*(\d\d\d\d)\s*\t*(\d\d)\s*\t*(\d\d\d\d\d\d\d\d)";

pub(crate) struct Lib{
    
}

impl Lib {

    /// Check if an SWIFT is correct or not returning true or false
    /// Valid swift example: BBVAESMMXXX
    /// Invalid swift example: BVAEMMX
    pub fn check_swift(swift: String) -> bool{
        let reg_ex = Regex::new(REGEX_SWIFT).unwrap();
        reg_ex.is_match(&swift)
    }

    /// Check if an IBAN code is correct or not returning true or false
    /// Valid iban example: ES49998877666655554440
    /// Invalid iban example: XXX632505000002014145
    pub fn check_iban(iban: String) -> bool{
        let reg_ex = Regex::new(REGEX_IBAN).unwrap();
        reg_ex.is_match(&iban)
    }

}