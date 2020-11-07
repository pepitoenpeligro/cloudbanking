mod user;
mod bankaccount;
mod bankcard;
mod bankfund;
mod paymentgroup;
mod savinggroup;
mod controller;
mod utils;

use chrono::{NaiveDate, NaiveDateTime};
extern crate serde_json;



use crate::user::model::User;
use crate::bankaccount::model::bankaccount::*;
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};

use crate::controller::model::{CloudBankingController};



fn main() {


    let controller: CloudBankingController = CloudBankingController::new();
    println!("Hi, controller was created OK {}", controller.to_json());

}