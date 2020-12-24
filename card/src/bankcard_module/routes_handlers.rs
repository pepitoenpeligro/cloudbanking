
use std::collections::HashMap;
use std::sync::{Arc,Mutex, RwLock};
use log::{debug, error, log_enabled, info, Level};
use actix_web::*;

use crate::bankcard_module::message::Msg;



pub async fn not_responding(req: HttpRequest) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());
    let mut mesg = Msg::new();
    mesg.msg = String::from("You are not allowed to be here");
    return Ok(HttpResponse::BadRequest().json(mesg));
}


pub async fn healthcheck(req: HttpRequest) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());
    let mut mesg = Msg::new();
    mesg.msg = String::from("I am alive");
    return Ok(HttpResponse::Ok().json(mesg));
}


pub async fn get_account( req: HttpRequest) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());
    let mut mesg = Msg::new();
    mesg.msg = String::from("Here is your account");
    return Ok(HttpResponse::Ok().json(mesg));    
}
