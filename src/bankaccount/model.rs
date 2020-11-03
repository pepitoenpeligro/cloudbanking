use chrono::{NaiveDate, NaiveDateTime};
use std::fmt;

extern crate serde;
extern crate serde_json;
use serde::{Serialize,Deserialize};

/// Bank Account model
#[derive(Serialize, Deserialize, Debug)]
pub struct Account{
    id: String,
    swift: String,
    iban:String,
    created_at: NaiveDateTime,
    status: bool,
}


impl Account{

    /// Allocates a Account object and initializes it so that it represent the bankaccount needed.
    /// # Usage
    /// ```
    /// let my_account = Account::new(id,swift,iban,date,status)
    /// ```
    pub fn new(new_id: String, new_swift: String, new_iban: String, new_created_at: NaiveDateTime, new_status: bool) -> Account {
        Account {
            id: new_id,
            swift: new_swift,
            iban: new_iban,
            created_at: new_created_at,
            status: new_status
        }
    }

    // Getter (inmutable) for id private attribute.
    pub fn id(&self) -> &String{
        &self.id
    }


    fn enable(){

    }

    fn disable(){

    }

    // The to_json method allows to account to produce it own JSON serialization
    pub fn to_json(&self)->String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}


impl fmt::Display for Account{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.swift)
    }
}