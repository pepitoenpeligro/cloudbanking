mod user;
mod bankaccount;
mod bankcard;
mod bankfund;
mod paymentgroup;
mod savinggroup;
mod controller;

use crate::user::model::User;
use crate::bankaccount::model::{Account};
use crate::bankcard::model::{Card};
use crate::bankfund::model::{Fund};
use crate::savinggroup::model::{Savinggroup};
use crate::paymentgroup::model::{Paymentgroup};

use crate::controller::model::{CloudBankingController};



fn main() {
    let u: User;
    let a: Account;
    let c: Card;
    let f: Fund;
    let s: Savinggroup;
    let p: Paymentgroup;
    let controller: CloudBankingController;

    println!("Welcome to Cloud Banking");
}
