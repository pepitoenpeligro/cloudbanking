use chrono::{NaiveDate, NaiveDateTime};
use std::fmt;

extern crate serde;
extern crate serde_json;
use serde::{Serialize,Deserialize};

/// Bank Account model
#[derive(Serialize, Deserialize, Debug)]
pub struct Account{
    /// Identification of account, it should be an identification intern of app that it's consistent with database identificator
    id: String,

    /// International bank code that identifies (worldwide) tha bank associated to account (better than BIC)
    /// Example of Swift: BSCHESMMXXX
    swift: String,

    /// International Bank Account Number only for Europe.
    /// Example of IBAN: ES32 6688 05 1111222233344
    iban:String,

    /// Timestamp. Represent timestamp which account was added in our application. For statisticals use.
    created_at: NaiveDateTime,

    /// Status of account. User can disable an account for not operating (not interesting account)
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
    pub fn get_id(&self) -> &String{
        &self.id
    }

    // Getter (inmutable) for swift private attribute.
    pub fn get_swift(&self) -> &String{
        &self.swift
    }

    // Getter (inmutable) for iban private attribute.
    pub fn get_iban(&self) -> &String{
        &self.iban
    }

    // Getter (inmutable) for created_at private attribute.
    pub fn get_date(&self) -> &String{
        &self.created_at
    }

    // Getter (inmutable) for status private attribute.
    pub fn get_status(&self) -> &String{
        &self.status
    }

    // Enables an account
    fn enable(){
        self.status = true
    }

    // Disables an account
    fn disable(){
        self.status = false
    }

    // The to_json method allows to account to produce it own JSON serialization
    pub fn to_json(&self)->String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}


 // The fmt method allows display all atributes of an Account
impl fmt::Display for Account{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.id, self.swift, self.iban, self.created_at, self.status)
    }
}