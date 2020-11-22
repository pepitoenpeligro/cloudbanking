extern crate regex;
use regex::Regex;
use std::str;

/// Regular expression patter to check a valid SWIFT code
static REGEX_SWIFT: &str = r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$";

/// Regular expression patter to check a valid IBAN code
static REGEX_IBAN: &str = r"([A-Z]{2})\s*\t*(\d\d)\s*\t*(\d\d\d\d)\s*\t*(\d\d\d\d)\s*\t*(\d\d)\s*\t*(\d\d\d\d\d\d\d\d)";

static REGEX_NUMBER: &str = r"^(5[1-5][0-9]{14}|2(22[1-9][0-9]{12}|2[3-9][0-9]{13}|[3-6][0-9]{14}|7[0-1][0-9]{13}|720[0-9]{12}))$";

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

     /// Check if a card number is correct or not returning true or false
    /// Example 5500 0000 0000 0004 <- Mastercard ^(5[1-5][0-9]{14}|2(22[1-9][0-9]{12}|2[3-9][0-9]{13}|[3-6][0-9]{14}|7[0-1][0-9]{13}|720[0-9]{12}))$
    /// Example 4111 1111 1111 1111 <- Visa ^4[0-9]{12}(?:[0-9]{3})?$
    pub fn check_number(number: String)-> bool{
        let reg_ex = Regex::new(REGEX_NUMBER).unwrap();
        reg_ex.is_match(&number)
    }

}