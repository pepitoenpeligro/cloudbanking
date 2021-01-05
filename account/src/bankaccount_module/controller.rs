
    use std::sync::{Arc,Mutex, RwLock};
    use std::collections::HashMap;
    use serde::{Serialize,Deserialize};
    use std::fmt;

    
    use chrono::{Local, NaiveDate, NaiveDateTime};

    use crate::bankaccount_module::model::bankaccount::{Account};
    use crate::bankaccount_module::*;
    use crate::utils::*;




    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct BankAccountController{
        bank_accounts: Arc<RwLock<HashMap<String,Account>>>
    }


    impl BankAccountController{

        pub fn new() ->  BankAccountController {
            BankAccountController {
                bank_accounts: Arc::new(RwLock::new(HashMap::new()))
            }
        }

        // HU 1: Add Bank Account as customer user
        pub fn add_bank_account(&mut self,c: Account){
            self.bank_accounts.write().unwrap().insert(c.get_id().to_string(), c);
        }

        // HU 2: Delete bank account as customer user
        pub fn erase_bank_account(&mut self, bank_accout_id: String){
            self.bank_accounts.write().unwrap().remove(&bank_accout_id);
        }

        pub fn get_bank_accounts(&self) -> &Arc<RwLock<HashMap<String,Account>>> {
            &self.bank_accounts
        }


        /// The to_json method allows to account to produce it own JSON serialization
        pub fn to_json(&self)->String {
            serde_json::to_string_pretty(&self).unwrap()
        }
    }

    /// The fmt method allows display all atributes of an BankAccountController
    impl fmt::Display for BankAccountController{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:?})", self.bank_accounts)
        }
    }

