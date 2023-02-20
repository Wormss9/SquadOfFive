use crate::authorization::Login;
use crate::handlers;
use crate::utils::json_body;
use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::reply::{WithHeader, WithStatus};
use warp::{Filter, Rejection};

use super::{add_struct, auth_validation};

pub fn get_login(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (WithHeader<String>,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(add_struct(pool))
        .and(add_struct(key))
        .and_then(handlers::login::login)
}

pub fn put_register(
    pool: Pool,
) -> impl Filter<Extract = (WithStatus<String>,), Error = Rejection> + Clone {
    warp::put()
        .and(warp::path("api"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(add_struct(pool))
        .and_then(handlers::login::register)
}

pub fn get_restricted(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (WithStatus<String>,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("restricted"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and_then(handlers::login::restricted_handler)
}
