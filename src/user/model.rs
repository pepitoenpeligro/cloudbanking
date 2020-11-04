

pub mod User{

    use chrono::{NaiveDateTime};
    use std::fmt;
    use std::collections::HashMap;
    use serde::{Serialize,Deserialize};


    use crate::bankaccount::model::bankaccount::{Account};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User{
        id: String,
        email: String,
        hash_password:String,
        created_at: NaiveDateTime,
        bank_accounts: HashMap<String,Account>,
    
    }
    impl User {

        pub fn new(new_id: String, new_email: String, new_hash_password: String, new_created_at: NaiveDateTime) ->  User {
            User {
                id: new_id,
                email: new_email,
                hash_password: new_hash_password,
                created_at: new_created_at,
                bank_accounts: HashMap::new()
            }
        }

        /// Getter (inmutable) for id private attribute.
        pub fn get_id(&self) -> &String{
            &self.id
        }

        /// Getter (inmutable) for email private attribute.
        pub fn get_email(&self) -> &String{
            &self.email
        }

        /// Muttable access for email private attribute
        pub fn set_email(&mut self, new_email: String) -> &String{
            self.email=new_email;
            &self.email
        }

        /// Getter (inmutable) for hash_password private attribute.
        pub fn get_hash_password(&self) -> &String{
            &self.hash_password
        }

        /// Muttable access for hash_password private attribute
        pub fn set_hash_password(&mut self, new_hash_password: String) -> &String{
            self.hash_password=new_hash_password;
            &self.hash_password
        }

        /// Getter (inmutable) for hash_password private attribute.
        pub fn get_date(&self) -> &NaiveDateTime{
            &self.created_at
        }




    
        /// Scenario HU4
        pub fn delete_bank_account(){
    
        }
    
        /// Scenario HU2
        pub fn add_bank_card(){
    
        }
        
        /// Scenario HU5
        pub fn delete_bank_card(){
    
        }
    
        /// Scenario HU3
        pub fn add_fund_investment(){
    
        }
    
        /// Scenario HU6
        pub fn delete_fund_investment(){
            
        }
    
        /// Scenario HU7
        pub fn create_saving_group(){
    
        }
    
        /// Scenario HU8
        pub fn exit_saving_group(){
    
        }
    
        /// Scenario HU9
        pub fn create_payment_group(){
    
        }
    
        /// Scenario HU10
        pub fn exit_payment_group(){
    
        }

        /// The to_json method allows to account to produce it own JSON serialization
        pub fn to_json(&self)->String {
            serde_json::to_string_pretty(&self).unwrap()
        }
    }   
    
    /// The fmt method allows display all atributes of an Account
    impl fmt::Display for User{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {}, {}, {:?})", self.id, self.email, self.hash_password, self.created_at, self.bank_accounts)
        }
    }
}
