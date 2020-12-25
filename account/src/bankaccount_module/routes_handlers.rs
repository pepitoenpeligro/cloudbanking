
use warp::*;
use std::collections::HashMap;
use std::sync::{Arc,Mutex, RwLock};
// use log::{debug, error, log_enabled, info, Level};

use crate::bankaccount_module::message::Msg;

pub async fn get_accunts_handler(s: String) -> Result<impl warp::Reply, warp::Rejection>{
    let mut mesg = Msg::new();
    mesg.msg = String::from("okk");
    let response = Ok(warp::reply::with_status(warp::reply::json(&mesg), http::StatusCode::OK));
    println!("ok");
    return response;

    // let result = c.get_users();
    // let users = result.read().unwrap();

    // // If there is at least 1 User
    // if users.len() > 0 {
    //     let response = Ok(warp::reply::with_status(warp::reply::json(result), http::StatusCode::OK));
    //     println!("ok");
    //     return response;
    // }else{
    //     let response = Ok(warp::reply::with_status(warp::reply::json(result), http::StatusCode::NOT_FOUND));
    //     println!("err");
    //     return response;
    // }
}


// pub async fn not_responding(req: HttpRequest) -> Result<HttpResponse, Error> {
//     log::info!("Received request from uri: {}", req.uri());
//     let mut mesg = Msg::new();
//     mesg.msg = String::from("You are not allowed to be here");
//     return Ok(HttpResponse::BadRequest().json(mesg));
// }


// pub async fn healthcheck(req: HttpRequest) -> Result<HttpResponse, Error> {
//     log::info!("Received request from uri: {}", req.uri());
//     let mut mesg = Msg::new();
//     mesg.msg = String::from("I am alive");
//     return Ok(HttpResponse::Ok().json(mesg));
// }


// pub async fn get_account( req: HttpRequest) -> Result<HttpResponse, Error> {
//     log::info!("Received request from uri: {}", req.uri());
//     let mut mesg = Msg::new();
//     mesg.msg = String::from("Here is your account");
//     return Ok(HttpResponse::Ok().json(mesg));    
// }
