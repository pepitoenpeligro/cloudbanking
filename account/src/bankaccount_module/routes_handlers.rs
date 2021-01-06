


use warp::{Reply, Rejection};
use warp::http;
use std::collections::HashMap;
use std::sync::{Arc,Mutex, RwLock};
use std::{env, io};


use chrono::{Local, NaiveDate, NaiveDateTime};
use crate::bankaccount_module::controller::{BankAccountController};
use crate::bankaccount_module::message::Msg;
use crate::bankaccount_module::model::bankaccount::{Account};

pub async fn get_accounts_handler(id: String, mut bac: BankAccountController) -> Result<impl warp::Reply, warp::Rejection>{
    log::info!("[Get] /account/:id received -> get_accounts_handler");

    let result = bac.get_bank_accounts();
    let users = result.read().unwrap();

    // If there is at least 1 User
    if users.len() > 0 {
        let response = Ok(warp::reply::with_status(warp::reply::json(result), http::StatusCode::OK));
        println!("ok");
        log::info!("User returned successfully");
        return response;
    }else{
        let response = Ok(warp::reply::with_status(warp::reply::json(result), http::StatusCode::NOT_FOUND));
        log::info!("Error, there is no user with this id");
        return response;
    }
}



// HU 1: add bank account
pub async fn add_accounts_handler(c: Account, mut bac: BankAccountController) -> Result<impl warp::Reply, warp::Rejection>{
    log::info!("[POST] /account/ received -> add_accounts_handler");
    let mut mesg = Msg::new();
    bac.add_bank_account(c);
    mesg.msg = format!("{}", "Bank account added successfully");
    let response = Ok(warp::reply::with_status(warp::reply::json(&mesg), http::StatusCode::OK));
    log::info!("{}", mesg.msg);
    return response;
}


// HU 4: Erase bank account
pub async fn delete_accounts_handler(id: String, mut bac: BankAccountController) -> Result<impl warp::Reply, warp::Rejection>{
    log::info!("[DELETE] /account/ received -> delete_accounts_handler");
    let mut mesg = Msg::new();
    bac.erase_bank_account(id);
    mesg.msg = format!("{}", "Bank account deleted successfully");
    let response = Ok(warp::reply::with_status(warp::reply::json(&mesg), http::StatusCode::OK));
    log::info!("{}", mesg.msg);
    return response;
}

