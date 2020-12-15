use crate::user::model::user::User;
use warp::{Filter};

// pub fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

pub fn post_json() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}