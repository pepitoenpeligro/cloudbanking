pub mod user{

    use chrono::{NaiveDateTime};
    use std::fmt;
    use std::collections::HashMap;
    use serde::{Serialize,Deserialize};


    use crate::bankaccount::model::bankaccount::{Account};
    use crate::bankcard::model::bankcard::{Card};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct User{
        id: String,
        email: String,
        created_at: NaiveDateTime,
        bank_accounts: HashMap<String,Account>,
        bank_cards: HashMap<String,Card>,
    
    }
    impl User {

        pub fn new(new_id: String, new_email: String, new_created_at: NaiveDateTime) ->  User {
            User {
                id: new_id,
                email: new_email,
                created_at: new_created_at,
                bank_accounts: HashMap::new(),
                bank_cards: HashMap::new()
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
        pub fn get_date(&self) -> &NaiveDateTime{
            &self.created_at
        }


        /// Getter (inmutable) for bank_accounts private attribute.
        pub fn get_bank_accounts(&self) -> &HashMap<String,Account>{
            &self.bank_accounts
        }
        
    
        /// Scenario HU1
        pub fn add_bank_account(&mut self, account: Account) -> usize{
            self.bank_accounts.insert(String::from(account.get_id()), account);
            self.bank_accounts.len()
        }

    
        /// Scenario HU4
        pub fn delete_bank_account(&mut self, account_id: String) -> usize{
            self.bank_accounts.remove(&account_id);
            self.bank_accounts.len()
        }

        
        /// Scenario HU2
        pub fn add_bank_card(&mut self, card: Card) -> usize{
            self.bank_cards.insert(String::from(card.get_id()), card);
            self.bank_cards.len()
        }
        
        /// Scenario HU5
        pub fn delete_bank_card(&mut self, card_id:String) -> usize{
            self.bank_cards.remove(&card_id);
            self.bank_cards.len()
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
            write!(f, "({}, {}, {}, {:?})", self.id, self.email, self.created_at, self.bank_accounts)
        }
    }
}
