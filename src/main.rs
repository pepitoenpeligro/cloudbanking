mod user;
mod bankaccount;
mod bankcard;
mod bankfund;
mod paymentgroup;
mod savinggroup;
mod controller;
use chrono::{NaiveDate, NaiveDateTime};
extern crate serde_json;

use crate::user::model::User;
use crate::bankaccount::model::{Account};
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};

use crate::controller::model::{CloudBankingController};



fn main() {
    let u: User;
    let a: Account =  Account::new(String::from("0"), String::from("swift"), String::from("iban"), NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0), true);
    let c: Card;
    let f: Fund;
    let s: Savinggroup;
    let p: Paymentgroup;
    let controller: CloudBankingController;

    println!("Welcome to Cloud Banking - user, account, card, fund, savinggroup, paymentgroup, and cloudbankingcontroller are created");
    println!("{}", a.to_json());
    println!("{}", serde_json::to_string_pretty(&a).unwrap());

}