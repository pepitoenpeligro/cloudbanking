#![allow(dead_code, unused_variables, unused_comparisons, unused_assignments,unused_mut)]
mod bankcard_module;


extern crate dotenv;
extern crate serde_json;
use std::path::Path;


use chrono::{Local, NaiveDate, NaiveDateTime};
use actix::prelude::*;
use actix_web::{get, web,http, App, HttpServer, Responder, middleware, HttpResponse};
// use controller::middleware::{CheckIdUserService};
use std::{env, io};

use dotenv::dotenv;
use std::sync::{Arc,Mutex, RwLock};
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;
use std::io::Write;

use crate::bankcard_module::routes_handlers::*;

use actix_web::http::ContentEncoding;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use etcd_client::{Client, Error};

// cargo build --manifest-path=card/Cargo.toml
// cargo run -p card


const VERSION_ENV: &str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let mut port: String = String::from("3032");
    let mut host: String = String::from("127.0.0.1");


    // @TODO Cambiar prueba por account
    let my_path = env::current_dir()?.join("prueba/.env");
    log::info!("Reading environment from: {:?}", my_path);
    dotenv::from_path(my_path.as_path()).ok();
    dotenv().ok();



    port = env::var("PORT").expect("PORT must be set");
    host = env::var("HOST").expect("HOST must be set");

    let binding_uri = format!("{}:{}",host, port);
    


    let server = HttpServer::new(move   | | {
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

            // route GET /bank/cards
            .route("/bank/cards", web::get().to(get_account))
        )

    });

    // Enables HTTP2.0
    // It's required to request with public ssl cert
    // Works in local
    /*let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();*/
    log::info!("Welcome to bank/cards API {}", VERSION_ENV);
    log::info!("Server is listening in {}", binding_uri);
    //server.bind_openssl(binding_uri, builder)
    server.bind(binding_uri)
        .expect("Cannot bind to port")
        .run()
        .await

}