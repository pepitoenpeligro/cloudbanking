use warp::{http};
use std::collections::HashMap;
use crate::CloudBankingController;
use crate::user::model::user::User;



/// Handler 
/// Responds to requests in path /users
/// HTTP method : GET
/// Response in: JSON
pub async fn get_users_handler(c: CloudBankingController) -> Result<impl warp::Reply, warp::Rejection>{

    let result = c.get_users();
    let users = result.read().unwrap();

    // If there is at least 1 User
    if users.len() > 0 {
        let response = Ok(warp::reply::with_status(warp::reply::json(result), http::StatusCode::OK));
        println!("ok");
        return response;
    }else{
        let response = Ok(warp::reply::with_status(warp::reply::json(result), http::StatusCode::NOT_FOUND));
        println!("err");
        return response;
    }
}


pub async fn post_user_handler(u: User, mut c: CloudBankingController) -> Result<impl warp::Reply, warp::Rejection>{
    c.create_new_user(u);
    let result = "ok";
    Ok(warp::reply::json(
        &result
    ))
}

