use super::{add_struct, cookie_auth, utils::{json_body, json_query}};
use crate::handlers::{authorization, signing};
use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::{Filter, Rejection, Reply};

fn get_login(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
    .and(warp::path("api"))
    .and(warp::path("login"))
    .and(warp::path("user"))
    .and(warp::path::end())
    .and(json_query::<authorization::Login>())
    .and(add_struct(pool))
    .and(add_struct(key))
    .and_then(signing::user_login)
}

fn put_register(pool: Pool) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::put()
        .and(warp::path("api"))
        .and(warp::path("register"))
        .and(warp::path("user"))
        .and(warp::path::end())
        .and(json_body::<authorization::Login>())
        .and(add_struct(pool))
        .and_then(signing::user_register)
}

fn get_restricted(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("restricted"))
        .and(warp::path::end())
        .and(cookie_auth(key))
        .and_then(signing::restricted_handler)
}

pub fn signing(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    get_login(pool.clone(), key.clone())
        .or(put_register(pool))
        .or(get_restricted(key))
}
