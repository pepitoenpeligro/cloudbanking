#![allow(unused_imports,dead_code, unused_variables, unused_comparisons, unused_assignments,unused_mut)]
mod bankaccount_module;


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
use warp::Filter;


use crate::bankaccount_module::routes_handlers::*;

use etcd_client::{Client, Error};

// cargo build --manifest-path=prueba/Cargo.toml
// cargo run -p prueba


const VERSION_ENV: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    // std::env::set_var("RUST_LOG", "actix_web=debug");

    // env_logger::Builder::from_env(Env::default().default_filter_or("info").write_style_or("auto", "always"))
    // .format(|buf, record| {
    //     writeln!(
    //         buf,
    //         "{} {}: {}",
    //         record.level(),
    //         //Format like you want to: <-----------------
    //         Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
    //         record.args()
    //     )
    // })
    // .init();

    let mut port: String = String::from("3031");
    let mut host: String = String::from("0.0.0.0");


    // @TODO Cambiar account por account
    let my_path = env::current_dir().unwrap().join("account/.env");
    println!("Reading environment from: {:?}", my_path);
    dotenv::from_path(my_path.as_path()).ok();
    dotenv().ok();



    port = env::var("PORT").expect("PORT must be set");
    host = env::var("HOST").expect("HOST must be set");


    
    /*let server = HttpServer::new(move   | | {
        App::new()

        // Injecting controller to service (api calls)
        // .data(cbc.clone())

        // Defining Logger as middleware {INFO, DEBUG and ERROR} for actix-web logs
        .wrap(middleware::Logger::default())

        // Defining default Compress level for data exchange
        .wrap(middleware::Compress::new(ContentEncoding::Gzip))

        // Only accept GET, PUT, POST and DELETE verbs
        .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Methods", "GET, PUT, POST, DELETE"))

        // No access the site if detected XSS attempt. Not used now. WebBrowser needed
        .wrap(middleware::DefaultHeaders::new().header("X-XSS-Protection", "1; mode=block"))

        // We expect data exchange in only json format
        .wrap(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))

        // Preventing to any website from embedding. Not used now. WebBrowser needed
        .wrap(middleware::DefaultHeaders::new().header("X-Frame-Options","Deny"))

        // What type of content and origin we will allo.
        .wrap(middleware::DefaultHeaders::new().header("Content-Security-Policy","script-src 'self'"))

        // Preventing CSRF attacks
        .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Headers", "X-Requested-Width"))
        // For restrict client with mandatory use of HTTPS 
        // .wrap(middleware::DefaultHeaders::new().header("Strict-Transport-Security","max-age=31536000; includeSubDomains"))

        // /api/users
        .service(web::scope("/api")

            // route GET /bank/accounts
            .route("/bank/accounts", web::get().to(get_account))
        )

    });*/
    let control : String = "".to_string();
    let cbc_filter = warp::any().map(move | | control.clone());//|| cbc.clone());
    let socket : SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port.parse::<u16>().unwrap());


    let get_accounts = warp::get()
    .and(warp::path("accounts"))
    .and(warp::path::end())
    .and(cbc_filter.clone())
    .and_then(get_accunts_handler);

    let routes = get_accounts;//.or(post_account);


    println!("Welcome to bank/accounts API {}", VERSION_ENV);
    println!("Server is listening in {}", socket);
    warp::serve(routes).run(socket).await;


}