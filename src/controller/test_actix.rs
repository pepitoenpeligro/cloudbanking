

#[cfg(test)]
mod web_actix_test {

    extern crate actix_web;
    extern crate serde_json;
    extern crate actix_rt;
    extern crate hyper;

    use super::*;
    use actix_web::http::{header, StatusCode};
    use actix_web::{test, web, App, http, HttpServer, Responder, middleware, HttpResponse, HttpMessage};
    use hyper::Request;
    use std::sync::{Arc,Mutex, RwLock};
    use actix_web::http::ContentEncoding;

    use serde_json::{Value, json};
    use actix_web::test::TestRequest;


    use galvanic_assert::*;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::{bankaccount::model::bankaccount::*, controller::routes_handlers::*};
    use crate::user::model::user::User;
    use crate::bankcard::model::bankcard::*;
    use crate::utils::model::{Lib};
    use crate::controller::model::{CloudBankingController};


    /// Test Healthcheck route
    #[actix_rt::test]
    async fn test_healthcheck(){
        let req = test::TestRequest::with_header("content-type", "application/json")
            .uri("/healthcheck").to_http_request();


        let cbc :Arc<RwLock<CloudBankingController>>  = Arc::new(RwLock::new(CloudBankingController::new()));

        let mut app = test::init_service(
            App::new()
    
            // Injecting controller to service (api calls)
            .data(cbc.clone())
    
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
            // For restrict client with mandatory use of HTTPS 
            // .wrap(middleware::DefaultHeaders::new().header("Strict-Transport-Security","max-age=31536000; includeSubDomains"))
    
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
        ).await;

        let req = test::TestRequest::get().uri("/healthcheck").to_request();

        let resp = test::call_service(& mut app, req).await;
        println!("Response: {}", resp.status());
        assert!(resp.status().is_success());
    }


    /// POST /api/users
    /// Testing add user in system 
    /// [HU 14] Create account in system
    #[actix_rt::test]
    async fn test_add_user(){

        
        let id_user         : String                    = String::from("999f7f66abf88ee70243988");
        let email_user      : String                    = String::from("test_add_user@ostfalia.de");
        let date_user       : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let user            : User                      = User::new(id_user,email_user,date_user);

        let req = test::TestRequest::post().uri("/api/users").set_json(&user).to_request();

        let cbc :Arc<RwLock<CloudBankingController>>  = Arc::new(RwLock::new(CloudBankingController::new()));

        let mut app = test::init_service(
            App::new()
    
            // Injecting controller to service (api calls)
            .data(cbc.clone())
    
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
            // For restrict client with mandatory use of HTTPS 
            // .wrap(middleware::DefaultHeaders::new().header("Strict-Transport-Security","max-age=31536000; includeSubDomains"))
    
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
        ).await;

        let resp = test::call_service(& mut app, req).await;
        println!("response from add user: {:?}", resp);
        assert!(resp.status().is_success());
    }



    /// DELETE /api/users
    /// Testing delete user from system 
    /// [HU 16] Erase Account
    #[actix_rt::test]
    async fn test_delete_user(){

        
        let id_user         : String                    = String::from("999f7f66abf88ee70243988");
        let email_user      : String                    = String::from("test_delete_user@ostfalia.de");
        let date_user       : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let user            : User                      = User::new(id_user,email_user,date_user);

        let req = test::TestRequest::post().uri("/api/users").set_json(&user).to_request();

        let cbc :Arc<RwLock<CloudBankingController>>  = Arc::new(RwLock::new(CloudBankingController::new()));

        let mut app = test::init_service(
            App::new()
    
            // Injecting controller to service (api calls)
            .data(cbc.clone())
    
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
            // For restrict client with mandatory use of HTTPS 
            // .wrap(middleware::DefaultHeaders::new().header("Strict-Transport-Security","max-age=31536000; includeSubDomains"))
    
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
        ).await;

        let resp = test::call_service(& mut app, req).await;
        println!("response from add user: {:?}", resp);
        assert!(resp.status().is_success());

        let my_uri = format!("/api/users/{}", user.get_id());
        let req = test::TestRequest::delete().uri(my_uri.as_str()).to_request();
        let nresp = test::call_service(& mut app, req).await;
        println!("response from delete user: {:?}", nresp);
        assert!(nresp.status().is_success());
    }

}