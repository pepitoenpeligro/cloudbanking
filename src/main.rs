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




use crate::bankaccount::model::bankaccount::*;
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};
use crate::user::model::User::User;
use crate::utils::model::{Lib};

use crate::controller::model::{CloudBankingController};

fn main() {
    let id      : String            = String::from("507f1f77bcf86cd799439011");
    let swift   : String            = String::from("BSCHESMMXXX");
    let iban    : String            = String::from("ES32668805111122223334");
    let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
    let status  : bool              = true;
    let acc     : Account           = Account::new(id, swift, iban, date, status);

    let id_user             : String                    = String::from("999f7f66abf88ee70243988");
    let email_user          : String                    = String::from("j.cordoba@ostfalia.de");
    let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
    let mut user            : User                      = User::new(id_user,email_user,date_user);


    let mut controller: CloudBankingController = CloudBankingController::new();
    println!("Hi, controller was created OK {}", controller.to_json());

    controller.create_new_user(user);

    println!("We had added a user in system: {}", controller.to_json());

}