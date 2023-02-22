use authorization::get_key;
use database::connect;
use filters::{game, login, room};
use serde_derive::{Deserialize, Serialize};
use warp::Filter;
use websocket::WsPlayers;

mod authorization;
mod database;
mod filters;
mod game_logic;
mod handlers;
mod rejection;
mod utils;
mod websocket;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Name {
    name: String,
}

#[tokio::main]
async fn main() {
    let db_pool = connect().await;
    let key = get_key();
    let rooms = WsPlayers::default();

    let login_routes = login::get_login(db_pool.clone(), key.clone())
        .or(login::put_register(db_pool.clone()))
        .or(login::get_restricted(key.clone()));

    let room_routes = room::create_room(db_pool.clone(), key.clone())
        .or(room::get_room(db_pool.clone(), key.clone()))
        .or(room::join_room(db_pool.clone(), key.clone()))
        .or(room::get_my_rooms(db_pool.clone(), key.clone()))
        .or(room::get_joined_rooms(db_pool.clone(), key.clone()))
        .or(room::get_players(db_pool.clone(), key.clone()));

    let play_route = game::join_game(db_pool.clone(), key.clone(), rooms);

    let routes = login_routes.or(room_routes).or(play_route);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
