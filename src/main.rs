#![allow(unused_imports, dead_code, unused_variables)]
mod user;
mod bankaccount;
mod bankcard;
mod bankfund;
mod paymentgroup;
mod savinggroup;
mod controller;
mod utils;

mod json;

use warp::*;

use chrono::{NaiveDate, NaiveDateTime};
extern crate serde_json;




use crate::bankaccount::model::bankaccount::*;
use crate::bankcard::model::bankcard::*;
use crate::bankfund::model::bankfund::*;
use crate::savinggroup::model::*;
use crate::paymentgroup::model::*;
use crate::user::model::user::*;
use crate::utils::model::*;
use crate::controller::model::*;
use crate::json::helpers::*;


use crate::controller::handler::*;
use std::net::{SocketAddr,IpAddr,Ipv4Addr};





const VERSION_ENV: &str = env!("CARGO_PKG_VERSION");
const PORT : u16 = 3030;







#[tokio::main]
async fn main() {
    let mut cbc :CloudBankingController  = CloudBankingController::new();
    let cbc_filter = warp::any().map(move || cbc.clone());
    
    let socket : SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);



    let get_users = warp::get()
    .and(warp::path("users"))
    .and(warp::path::end())
    .and(cbc_filter.clone())
    .and_then(get_users_handler);


    let post_user = warp::post()
        .and(warp::path("users"))
        .and(warp::path::end()).and(post_json())
        .and(cbc_filter.clone())
        .and_then(post_user_handler);

    let routes = get_users.or(post_user);

    println!("Server started in {}",socket);
    warp::serve(routes).run(socket).await;
    
}