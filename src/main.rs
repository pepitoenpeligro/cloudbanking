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

extern crate dotenv;
extern crate serde_json;

use chrono::{NaiveDate, NaiveDateTime};
use actix::prelude::*;
use actix_web::{get, web,http, App, HttpServer, Responder, middleware, HttpResponse};
use std::{env, io};
use dotenv::dotenv;
use std::sync::{Arc,Mutex, RwLock};
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;


use crate::bankaccount::model::bankaccount::*;
use crate::bankcard::model::bankcard::*;
use crate::bankfund::model::bankfund::*;
use crate::savinggroup::model::*;
use crate::paymentgroup::model::*;
use crate::user::model::user::*;
use crate::utils::model::*;
use crate::controller::model::*;
use crate::json::helpers::*;
use crate::controller::routes_handlers::*;



// Better with ETCD3 client
const VERSION_ENV: &str = env!("CARGO_PKG_VERSION");


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();

    let port = env::var("PORT").expect("PORT must be set");
    let host = env::var("HOST").expect("HOST must be set");

    let binding_uri = format!("{}:{}",host, port);
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Mutex over Controller Object
    let cbc :Arc<RwLock<CloudBankingController>>  = Arc::new(RwLock::new(CloudBankingController::new()));

    let server = HttpServer::new(move   | | {
        App::new()

        // Injecting controller to service
        .data(cbc.clone())

        // Defining Logger as middleware {INFO, DEBUG and ERROR} for actix-web logs
        .wrap(middleware::Logger::default())

        // Defining default Compress level for data exchange
        .wrap(middleware::Compress::default())
        // /api/users
        .service(web::scope("/api")

            // route GET /api/users
            .route("/users", web::get().to(get_users))
            // route POST /api/users
            .route("/users", web::post().to(add_user))

            // route GET /api/users/{id}
            .route("/users/{id}", web::get().to(get_user_by_id))


            .route(
                "/no",web::method(http::Method::HEAD)
                    .to(not_responding),
            )

        )
        // /_ We can let it for static files
        .service(web::scope("/")
            .route("/healthcheck", web::get().to(healthcheck))
    )
        
    });
    println!("Server is listening in {}", binding_uri);
    server.bind(binding_uri)
        .expect("Cannot bind to port")
        .run()
        .await
}