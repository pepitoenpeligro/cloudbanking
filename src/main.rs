#![allow(unused_imports, dead_code, unused_variables)]
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
use crate::bankcard::model::bankcard::*;
use crate::bankfund::model::bankfund::*;
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};
use crate::user::model::user::User;
use crate::utils::model::{Lib};
use crate::controller::model::{CloudBankingController};

const VERSION_ENV: &str = env!("CARGO_PKG_VERSION");


fn main() {
    let id      : String            = String::from("507f1f77bcf86cd799439011");
    let swift   : String            = String::from("BSCHESMMXXX");
    let iban    : String            = String::from("ES32668805111122223334");
    let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
    let status  : bool              = true;
    let acc     : Account           = Account::new(id, swift, iban, date, status);

    let id      : String            = String::from("507f1f77bcf86cd799439011");
    let card    : String            = String::from("5555111122223333");
    let cvc     : String            = String::from("584");
    let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
    let status  : bool              = true;
    let bc      : Card              = Card::new(id, card, cvc, date, status);

    let id_user             : String                    = String::from("999f7f66abf88ee70243988");
    let email_user          : String                    = String::from("j.cordoba@ostfalia.de");
    let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
    let mut user                : User                      = User::new(id_user,email_user,date_user);



    let id          : String            = String::from("507f1f77bcf86cd799439011");
    let amount      : f64               = 534.4;
    let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
    let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
    let status      : bool              = true;

    let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

    println! ("HU covered in this release ({}):", VERSION_ENV);
    println! ("\t -[x] HU1 Add bank account as customer user");
    println! ("\t -[x] HU4 Delete bank account as customer user ");
    println! ("\t -[x] HU16 Erase Account ");
    println! ("\t -[x] HU14 Create user");

    println! ("\t -[x] HU2 Add bank-card as customer user");
    println! ("\t -[x] HU5 Delete bank card as customer user");

    println! ("\t -[x] HU3 Add investment fund");
    println! ("\t -[x] HU6 Delete investment fund");
    println!("\n\n\n");
    
    let mut controller: CloudBankingController = CloudBankingController::new();
    println!("Hi, controller was created OK {}", controller.to_json());
    controller.create_new_user(user.clone());
    println!("We had added a user in system: {}", controller.to_json());
    println!("We delete now this user from system");
    controller.erase_account(String::from("999f7f66abf88ee70243988"));
    println!("Now our customer users are: {} ", controller.to_json());
    println!("Adding Card to User:");
    user.add_bank_card(bc);
    println!("Now Card is added to user {}", user.to_json());
    println!("Removing Card from User:");
    user.delete_bank_card(String::from("507f1f77bcf86cd799439011"));
    println!("Now Card is removed from user {}", user.to_json());
    println!("Adding bank fund {}", bf);
    user.add_fund_investment(bf.clone());

    println!("Erasing bank fund {}", bf);
    user.delete_fund_investment(bf.get_id().clone());

    

}