use std::sync::{Arc,Mutex, RwLock};

use crate::user::model::User::User;
use crate::bankaccount::model::bankaccount::*;
use std::collections::HashMap;
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};


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
    fn create_alert_trigger_bank_account(){

    }

    // Scenario HU 12
    fn create_alert_trigger_bank_card(){
        
    }

    // Scenario HU 13
    fn create_alert_trigger_bank_fund(){
        
    }

    // Scenario HU 14
    fn register_new_user(){

    }

    // Scenario HU 15
    fn login_user(){

    }

    // Scenario HU 16
    fn erase_account(){

    }

    // Scenario HU 17
    fn disable_account(){

    }

    // Scenario HU 18
    fn enable_disabled_account(){

    }
}