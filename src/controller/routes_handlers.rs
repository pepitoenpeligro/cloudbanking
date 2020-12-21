
use std::collections::HashMap;
use crate::CloudBankingController;
use crate::user::model::user::User;
use std::sync::{Arc,Mutex, RwLock};

use std::cell::RefCell;
use std::rc::Rc;


use log::{debug, error, log_enabled, info, Level};
use actix_web::*;



use super::middleware::*;
use crate::controller::message::*;


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


pub async fn get_users(cbc: web::Data<Arc<RwLock<CloudBankingController>>>, req: HttpRequest) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());

    let my_controller = cbc.read();
    let users_lock= my_controller.unwrap();
    let users_arc = users_lock.get_users();
    let users = users_arc.read().unwrap();

    if users.len() >= 0 {
        log::info!("Returning users list {:?}", users.to_owned());
        return Ok(HttpResponse::Ok().json(users.to_owned()));
    }else{
        log::info!("Error getting user list {:?}", users.to_owned());
        //return Ok(HttpResponse::NoContent().json(users.to_owned()));
        return Ok(HttpResponse::NotFound().json(users.to_owned()));
    }

    
}


pub async fn get_user_by_id(_: CheckIdUserService, cbc: web::Data<Arc<RwLock<CloudBankingController>>>, req: HttpRequest, user_id: web::Path<String>) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());
    log::info!("Id to search {}", user_id);
    let my_controller = cbc.read();
    let users_lock= my_controller.unwrap();
    let users_arc = users_lock.get_users();
    let users = users_arc.read().unwrap();
    

    if users.contains_key(user_id.as_str()) {
        return Ok(HttpResponse::Ok().json(users.get(user_id.as_str())));
    }
    let mut mesg = Msg::new();
    mesg.msg = String::from("user id not found. Please create user before search");
    return Ok(HttpResponse::NotFound().json(mesg));
}

pub async fn delete_user_by_id(_: CheckIdUserService, cbc: web::Data<Arc<RwLock<CloudBankingController>>>, req: HttpRequest, user_id: web::Path<String>) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());

    let users_in_system = cbc.read().unwrap().get_users().read().unwrap().to_owned();

    let user_id_key = user_id.as_str();
    println!("Searching user id in system: {} ",user_id_key );

    if users_in_system.contains_key(user_id_key) {
        log::info!("id is in system");    
        cbc.write().unwrap().erase_account(String::from(user_id_key));
        log::info!("user {} erased sucessfully", user_id_key);
        let mut mesg = Msg::new();
        mesg.msg = String::from("user deleted");
        return Ok(HttpResponse::Ok().json(mesg));

    }
    log::info!("user id is not in controller. Not possible to erase");
    //return Ok(HttpResponse::NoContent().json("It's not possible to erase user. User id not found. Please create user before search"));
    let mut mesg = Msg::new();
    mesg.msg = String::from("It's not possible to erase user. User id not found. Please create user before search");
    return Ok(HttpResponse::NotFound().json(mesg));
    
}



pub async fn update_user_by_id(_: CheckIdUserService, cbc: web::Data<Arc<RwLock<CloudBankingController>>>, user: web::Json<User>, req: HttpRequest, user_id: web::Path<String>) -> Result<HttpResponse, Error>{
    log::info!("Received request from uri: {}", req.uri());

    let users_in_system = cbc.read().unwrap().get_users().read().unwrap().to_owned();

    let user_id_key = user_id.as_str();
    println!("Searching user id in system: {} ",user_id_key );

    if users_in_system.contains_key(user_id_key) {
        log::info!("id is in system");
        // Updating the key user_id_key 
        cbc.write().unwrap().create_new_user(user.to_owned());
        log::info!("user {} updated sucessfully", user_id_key);

        let mut mesg = Msg::new();
        mesg.msg = String::from("user updated");
        return Ok(HttpResponse::Created().json(mesg));
    }

    log::info!("user id is not in controller. Not possible to update");

    let mut mesg = Msg::new();
    mesg.msg = String::from("It's not possible to update user. User id not found. Please create user before search");
    return Ok(HttpResponse::NoContent().json(mesg));



}

pub async fn add_user(cbc: web::Data<Arc<RwLock<CloudBankingController>>> , request: web::Json<User>, req: HttpRequest) -> Result<HttpResponse, Error> {
    log::info!("Received request from uri: {}", req.uri());
    let user :User = request.to_owned();
    cbc.write().unwrap().create_new_user(user);

    let mut mesg = Msg::new();
    mesg.msg = String::from("user saved");
    // return 201
    return Ok(HttpResponse::Created().json(mesg));
}

    