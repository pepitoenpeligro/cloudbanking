use std::sync::{Arc,Mutex, RwLock};

use crate::user::model::user::User;
use crate::bankaccount::model::bankaccount::*;
use std::collections::HashMap;
use crate::bankcard::model::bankcard::*;
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};
use serde::{Serialize,Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudBankingController{
    users: Arc<RwLock<HashMap<String,User>>>
}


impl CloudBankingController{

    pub fn new() ->  CloudBankingController {
        CloudBankingController {
            users: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    // Scenario HU 11
    pub fn create_alert_trigger_bank_account(){

    }

    // Scenario HU 12
    pub fn create_alert_trigger_bank_card(){
        
    }

    // Scenario HU 13
    pub fn create_alert_trigger_bank_fund(){
        
    }

    // Scenario HU 14
    pub fn create_new_user(&mut self, user: User){
        self.users.write().unwrap().insert(user.get_id().clone(), user);
    }


    // Scenario HU 16
    pub fn erase_account(&mut self, user_id: String){
        self.users.write().unwrap().remove(&user_id);

    }

    // Scenario HU 17
    pub fn disable_account(){

    }

    // Scenario HU 18
    pub fn enable_disabled_account(){

    }

    pub fn get_users(&self) -> &Arc<RwLock<HashMap<String,User>>> {
        &self.users
    }

    /// The to_json method allows to account to produce it own JSON serialization
    pub fn to_json(&self)->String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

/// The fmt method allows display all atributes of an CloudBankingController
impl fmt::Display for CloudBankingController{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.users)
    }
}