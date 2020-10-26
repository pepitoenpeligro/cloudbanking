use parking_lot::RwLock;
use std::sync::Arc;

use crate::user::model::{User};
use crate::bankaccount::model::{Account};
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};

pub struct CloudBankingController{
    users: Arc<RwLock<User>>
}


impl CloudBankingController{

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