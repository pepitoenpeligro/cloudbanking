use chrono::{NaiveDateTime};
use parking_lot::RwLock;
use std::sync::Arc;


use crate::bankaccount::model::{Account};
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};

pub struct User{
    id: String,
    email: String,
    password:String,
    created_at: NaiveDateTime,
    status: bool,
    bank_accounts: Arc<RwLock<Account>>,
    bank_cards: Arc<RwLock<Card>>,
    bank_funds: Arc<RwLock<Fund>>,
    saving_groups: Arc<RwLock<Savinggroup>>,
    payment_groups: Arc<RwLock<Paymentgroup>>

}

impl User {
    fn enable(){
        
    }

    fn disable(){

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
