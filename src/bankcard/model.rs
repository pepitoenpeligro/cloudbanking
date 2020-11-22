pub mod bankcard{
    use chrono::{NaiveDateTime};
    use std::fmt;
    use serde::{Serialize,Deserialize};
    use crate::utils::model::{Lib};

    extern crate regex;
    extern crate serde;
    extern crate serde_json;

    /// Bank Card model
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Card{
        /// Identification of a bank card, it should be an identification intern of app that it's consistent with database identificator
        id: String,

        /// Card Number
        /// Example 5500 0000 0000 0004 <- Mastercard ^(5[1-5][0-9]{14}|2(22[1-9][0-9]{12}|2[3-9][0-9]{13}|[3-6][0-9]{14}|7[0-1][0-9]{13}|720[0-9]{12}))$
        /// Example 4111 1111 1111 1111 <- Visa ^4[0-9]{12}(?:[0-9]{3})?$
        number: String,

        /// CVC
        /// Example 584
        cvc:String,

        /// Due Date until bank card is valid
        date_limit: NaiveDateTime,

        /// Current Status of card
        status: bool
    }

    impl Card{
        /// Allocates a Card object and initializes it so that it represent the card needed.
        /// /// # Usage
        /// ```
        /// let my_card = Card::new(id,number,cvc,date,status)
        /// ```
        pub fn new(new_id: String, new_number: String, new_cvc: String, new_date: NaiveDateTime, new_status:bool) -> Card{
            Card{
                id:new_id,
                number:new_number,
                cvc:new_cvc,
                date_limit:new_date,
                status:new_status,
            }
        }

        /// Getter (inmutable) for id private attribute
        pub fn get_id(&self) -> &String{
            &self.id
        }

        /// Getter (inmutable) for number private attribure.
        pub fn get_number(&self)-> &String{
            &self.number
        }

        /// Muttable access for number private attribute
        pub fn set_number(&mut self, new_card: String) -> &String{
            self.number = new_card;
            &self.number 
        }

        /// Getter (inmutable) for cvc private attribure.
        pub fn get_cvc(&self)-> &String{
            &self.cvc
        }

        /// Muttable access for cvc private attribute
        pub fn set_cvc(&mut self, new_cvc: String) -> &String{
            self.cvc = new_cvc;
            &self.cvc 
        }

        /// Getter (inmutable) for date private attribure.
        pub fn get_date(&self)-> &NaiveDateTime{
            &self.date_limit
        }

        /// Muttable access for date private attribute
        pub fn set_date(&mut self, new_date: NaiveDateTime) -> &NaiveDateTime{
            self.date_limit = new_date;
            &self.date_limit 
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

    /// The fmt method allows display all atributes of an Card
    impl fmt::Display for Card{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {}, {}, {})", self.id, self.number, self.cvc, self.date_limit, self.status)
        }
    }
}