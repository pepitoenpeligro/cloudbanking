

pub mod User{
    use chrono::{NaiveDateTime};
    use std::fmt;
    use std::collections::HashMap;
    use serde::{Serialize,Deserialize};


    use crate::bankaccount::model::bankaccount::*;

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
    
        // Scenario HU1
        fn add_bank_account(){
    
        }
    
        // Scenario HU4
        fn delete_bank_account(){
    
        }
    
        // Scenario HU2
        fn add_bank_card(){
    
        }
        
        // Scenario HU5
        fn delete_bank_card(){
    
        }
    
        // Scenario HU3
        fn add_fund_investment(){
    
        }
    
        // Scenario HU6
        fn delete_fund_investment(){
            
        }
    
        // Scenario HU7
        fn create_saving_group(){
    
        }
    
        // Scenario HU8
        fn exit_saving_group(){
    
        }
    
        // Scenario HU9
        fn create_payment_group(){
    
        }
    
        // Scenario HU10
        fn exit_payment_group(){
    
        }
    }    
}



