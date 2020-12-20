use std::sync::{Arc,Mutex, RwLock};

use log::{debug, error, log_enabled, info, Level};
use actix_web::*;

use actix_service::{Service, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use futures::Future;
use std::pin::Pin;

use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};

use std::task::{Context, Poll};

use std::collections::HashMap;
use crate::CloudBankingController;
use crate::user::model::user::User;
use serde::{Serialize,Deserialize};

extern crate regex;
use regex::Regex;


pub struct CheckIdUserService;

impl FromRequest for CheckIdUserService {
    type Error = Error;
    type Future = Ready<Result<CheckIdUserService, Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let parts = _req.uri().clone().into_parts().path_and_query.unwrap();
        log::info!("[Middleware CheckIdUserService] {:?}", parts);

        // Extract the last part of uri
        let string_reg_extract : &str = r"([^/]+$)";
        let reg_ex = Regex::new(string_reg_extract).unwrap();
        let parts_as_string = parts.as_str();

        if reg_ex.is_match(parts_as_string) {
            log::info!(" [2] valid id");
            return ok(CheckIdUserService);

        }
        log::info!(" [0] invalid id format");
        return err(ErrorUnauthorized("invalid id format"));
    }

}
