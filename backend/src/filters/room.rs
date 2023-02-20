use crate::handlers::room;
use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::reply::Json;
use warp::{Filter, Rejection};

use super::{add_struct, auth_validation};

pub fn create_room(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::put()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::create_room)
}

pub fn get_my_rooms(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("rooms"))
        .and(warp::path("created"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_my)
}

pub fn get_joined_rooms(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("rooms"))
        .and(warp::path("joined"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_joined)
}

pub fn get_players(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::param())
        .and(warp::path("players"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_players)
}

pub fn get_room(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_room)
}

pub fn join_room(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::patch()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::join_room)
}
