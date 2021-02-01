pub mod bankaccount{

    use chrono::{NaiveDateTime};
    use std::fmt;
    use serde::{Serialize,Deserialize};
    use crate::utils::model::{Lib};

    extern crate regex;
    extern crate serde;
    extern crate serde_json;


    /// Bank Account model
    #[derive(Serialize, Deserialize, Debug, Clone)]
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
        pub fn new(new_id: String, new_swift: String, new_iban: String, new_created_at: NaiveDateTime, new_status: bool) ->  Account {
            Account {
                id: new_id,
                swift: new_swift,
                iban: new_iban,
                created_at: new_created_at,
                status: new_status
            }
        }

        /// Getter (inmutable) for id private attribute.
        pub fn get_id(&self) -> &String{
            &self.id
        }

        /// Getter (inmutable) for swift private attribute.
        /// A bank could be fusioned with other bank and change a swift code
        pub fn get_swift(&self) -> &String{
            &self.swift
        }

        /// Muttable access for swift private attribute
        pub fn set_swift(&mut self, new_swift: String) -> &String{
            if Lib::check_swift(new_swift.clone()) {
                self.swift=new_swift;
            }
            
            &self.swift
        }

        /// Getter (inmutable) for iban private attribute.
        pub fn get_iban(&self) -> &String{
            &self.iban
        }

        /// Muttable access for swift private attribute
        /// A bank could be fusioned with other bank and change a iban code
        pub fn set_iban(&mut self, new_iban: String) -> &String{
            if Lib::check_iban(new_iban.clone()) {
                self.iban=new_iban;
            }
            &self.iban
        }


        /// Getter (inmutable) for created_at private attribute.
        pub fn get_date(&self) -> &NaiveDateTime{
            &self.created_at
        }

        /// Getter (inmutable) for status private attribute.
        pub fn get_status(&self) -> &bool{
            &self.status
        }

        /// Enables an account
        fn enable(&mut self) {
            self.status = true
        }

        /// Disables an account
        fn disable(&mut self) {
            self.status = false
        }

        /// The to_json method allows to account to produce it own JSON serialization
        pub fn to_json(&self)->String {
            serde_json::to_string_pretty(&self).unwrap()
        }

    }

    /// The fmt method allows display all atributes of an Account
    impl fmt::Display for Account{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {}, {}, {})", self.id, self.swift, self.iban, self.created_at, self.status)
        }
    }
}