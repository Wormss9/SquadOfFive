use super::{add_struct, header_auth};
use crate::{handlers::game, websocket::WsPlayers};
use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::{Filter, Rejection, Reply};

pub fn game(
    pool: Pool,
    key: Hmac<Sha512>,
    rooms: WsPlayers,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("api")
        .and(warp::path("game"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(header_auth(key))
        .and(add_struct(pool))
        .and(add_struct(rooms))
        .and(warp::ws())
        .map(game::game)
}
