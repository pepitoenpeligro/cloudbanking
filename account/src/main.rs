#![allow(unused_imports,dead_code, unused_variables, unused_comparisons, unused_assignments,unused_mut)]
mod bankaccount_module;
mod utils;


extern crate dotenv;
extern crate serde_json;
use std::path::Path;


use chrono::{Local, NaiveDate, NaiveDateTime};


// use controller::middleware::{CheckIdUserService};
use std::{env, io};

use dotenv::dotenv;
use std::sync::{Arc,Mutex, RwLock};
// use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;
use std::io::Write;
use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use warp::*;


use crate::bankaccount_module::*;
use crate::bankaccount_module::routes_handlers::*;
use crate::bankaccount_module::controller::*;
use crate::bankaccount_module::model::bankaccount::{Account};


use etcd_client::{Client, Error};

use warp::{Filter};

pub fn filter_json() -> impl Filter<Extract = (Account,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
    

// cargo run --manifest-path=./account/Cargo.toml


const VERSION_ENV: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    // std::env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::Builder::from_env(Env::default().default_filter_or("info").write_style_or("auto", "always"))
    .format(|buf, record| {
        writeln!(
            buf,
            "{} {}: {}",
            record.level(),
            //Format like you want to: <-----------------
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.args()
        )
    })
    .init();


    let mut port: String = String::from("3031");
    let mut host: String = String::from("0.0.0.0");

    let my_path = env::current_dir().unwrap().join("account/.env");
    println!("Reading environment from: {:?}", my_path);
    dotenv::from_path(my_path.as_path()).ok();
    dotenv().ok();



    port = env::var("PORT").expect("PORT must be set");
    host = env::var("HOST").expect("HOST must be set");


    let mut control : BankAccountController = BankAccountController::new();
    let cbc_filter = warp::any().map(move | | control.clone());
    let socket : SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port.parse::<u16>().unwrap());


    let get_accounts = warp::get()
        .and(warp::path!("accounts" / String))
        .and(warp::path::end())
        .and(cbc_filter.clone())
        //.and(warp::path::param())
        .and_then(get_accounts_handler);


    // HU 1: Add Bank Account as customer user
    let add_accounts = warp::post()
        .and(warp::path("accounts"))
        .and(warp::path::end())
        .and(filter_json())
        .and(cbc_filter.clone())
        .and_then(add_accounts_handler);

    // HU 4: Erase bank account    
    let delete_account = warp::delete()
        .and(warp::path!("accounts" / String))
        .and(warp::path::end())
        .and(cbc_filter.clone())
        .and_then(delete_accounts_handler);

    let routes = get_accounts
                .or(add_accounts)
                .or(delete_account);

    println!("Welcome to bank/accounts API {}", VERSION_ENV);
    println!("[Warp] Server is listening in {}", socket);
    
    warp::serve(routes).run(socket).await;
}