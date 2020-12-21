#![allow(unused_imports, dead_code, unused_variables, unused_comparisons, unused_assignments)]
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
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use etcd_client::{Client, Error};

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


    // By default. Only if:
    // 1. ETCD is not avaible
    // 2. .env file is not reachable or defined

    let mut port: String = String::from("3030");
    let mut host: String = String::from("127.0.0.1");
    
    let client_etcd = Client::connect(["localhost:2379"], None).await;

    if client_etcd.is_ok(){
        let mut client_unwrap = client_etcd.unwrap();
        let resp_etc_host = client_unwrap.get("HOST", None).await;
        let resp_etc_port = client_unwrap.get("PORT", None).await;

        host = String::from(resp_etc_host.unwrap().kvs().first().unwrap().value_str().unwrap().to_string().as_str());
        port = String::from(resp_etc_port.unwrap().kvs().first().unwrap().value_str().unwrap().to_string().as_str());

        log::info!("Host from etcd: {:?}", host);
        log::info!("Port from etcd: {:?}", port);
    }else{
        log::info!("No client etc, we will use .env");
    }
    
    
    
    dotenv().ok();

    port = env::var("PORT").expect("PORT must be set");
    host = env::var("HOST").expect("HOST must be set");

    let binding_uri = format!("{}:{}",host, port);
    
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

    // Enables HTTP2.0
    // It's required to request with public ssl cert
    // Works in local
    /*let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();*/
    log::info!("Welcome to cloudbanking API {}", VERSION_ENV);
    log::info!("Server is listening in {}", binding_uri);
    //server.bind_openssl(binding_uri, builder)
    server.bind(binding_uri)
        .expect("Cannot bind to port")
        .run()
        .await
}