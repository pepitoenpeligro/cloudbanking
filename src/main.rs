#![allow(unused_imports, dead_code, unused_variables, unused_comparisons)]
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

use chrono::{Local, NaiveDate, NaiveDateTime};
use actix::prelude::*;
use actix_web::{get, web,http, App, HttpServer, Responder, middleware, HttpResponse};
use controller::middleware::{CheckIdUserService};
use std::{env, io};
use dotenv::dotenv;
use std::sync::{Arc,Mutex, RwLock};
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;
use std::io::Write;

use actix_web::http::ContentEncoding;

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

    // Mutex over Controller Object
    let cbc :Arc<RwLock<CloudBankingController>>  = Arc::new(RwLock::new(CloudBankingController::new()));

    let server = HttpServer::new(move   | | {
        App::new()

        // Injecting controller to service (api calls)
        .data(cbc.clone())

        // Defining Logger as middleware {INFO, DEBUG and ERROR} for actix-web logs
        .wrap(middleware::Logger::default())

        // Defining default Compress level for data exchange
        .wrap(middleware::Compress::new(ContentEncoding::Gzip))

        // /api/users
        .service(web::scope("/api")

            // route GET /api/users
            .route("/users", web::get().to(get_users))

            // route POST /api/users
            .route("/users", web::post().to(add_user))

            // route GET /api/users/{id}
            .route("/users/{id}", web::get().to(get_user_by_id))

            // route DELETE /api/users/{id}
            .route("/users/{id}", web::delete().to(delete_user_by_id))
            
            // route PUT /api/users/{id}
            .route("/users/{id}",  web::put().to(update_user_by_id))

        )
        // /_ We can let it for static files
        .service(web::scope("/")
            .route("/healthcheck", web::get().to(healthcheck))
    )
        
    });
    log::info!("Server is listening in {}", binding_uri);
    server.bind(binding_uri)
        .expect("Cannot bind to port")
        .run()
        .await
}