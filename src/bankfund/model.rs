pub mod bankfund{
    use chrono::{NaiveDateTime};
    use std::fmt;
    use serde::{Serialize,Deserialize};
    use crate::utils::model::{Lib};

    extern crate regex;
    extern crate serde;
    extern crate serde_json;

    /// Bank Fund model
    pub struct Fund{
        /// Identification of the bank fund, it should be an identification intern of app that it's consistent with database identificator
        id: String,

        /// Amount invested
        amount: f64,

        /// Due Date when fund starts to be active
        date_start: NaiveDateTime,

        /// Due Date until fund is active
        date_finish: NaiveDateTime,
        /// Current Status of fund
        status: bool
    }

    impl Fund{

        /// Allocate a Fund object and intializes it so that it represent the fund investment
        /// Usage
        /// let my_fund = Fund::new(id,amount,date_start,date_finish,status)
        pub fn new(new_id: String, new_amount: f64, new_date_start: NaiveDateTime,  new_date_finish:NaiveDateTime , new_status:bool) -> Fund{
            Fund{
                id: new_id,
                amount: new_amount,
                date_start: new_date_start,
                date_finish: new_date_finish,
                status : new_status
            }
        }

        /// Getter (inmutable) for id private attribute
        pub fn get_id(&self) -> &String{
            &self.id
        }

        /// Getter (inmutable) for number private attribure.
        pub fn get_amount(&self)-> &f64{
            &self.amount
        }

        /// Muttable access for amount private attribute
        pub fn set_amount(&mut self, amount: f64) -> &f64{
            self.amount = amount;
            &self.amount 
        }

        /// Getter (inmutable) for date private attribure.
        pub fn get_start_date(&self)-> &NaiveDateTime{
            &self.date_start
        }

        /// Muttable access for date private attribute
        pub fn set_start_date(&mut self, new_date: NaiveDateTime) -> &NaiveDateTime{
            self.date_start = new_date;
            &self.date_start 
        }

        /// Getter (inmutable) for date private attribure.
        pub fn get_finish_date(&self)-> &NaiveDateTime{
            &self.date_finish
        }

        /// Muttable access for date private attribute
        pub fn set_finish_date(&mut self, new_date: NaiveDateTime) -> &NaiveDateTime{
            self.date_finish = new_date;
            &self.date_finish 
        }

        /// Inmutable access to check the status of the card
        pub fn is_active(&self) -> bool {
            self.status
        }

        /// Enables a card
        fn enable(&mut self) {
            self.status = true
        }

        /// Disables acard
        fn disable(&mut self) {
            self.status = false
        }

    }

    /// The fmt method allows display all atributes of an Bank
    impl fmt::Display for Fund{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {}, {}, {})", self.id, self.amount, self.date_start, self.date_finish, self.status)
        }
    }

}
