use authorization::get_key;
use database::connect;
use filters::{room,login};
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

mod authorization;
mod database;
mod filters;
mod game;
mod handlers;
mod rejection;
mod utils;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Name {
    name: String,
}

#[tokio::main]
async fn main() {
    let pool = connect().await;
    let key = get_key();

    let login_routes = login::get_login(pool.clone(), key.clone())
        .or(login::put_register(pool.clone()))
        .or(login::get_restricted(key.clone()));

    let room_routes = room::create_room(pool.clone(), key.clone())
        .or(room::get_room(pool.clone(), key.clone()))
        .or(room::join_room(pool.clone(), key.clone()))
        .or(room::get_my_rooms(pool.clone(), key.clone()))
        .or(room::get_joined_rooms(pool.clone(), key.clone()))
        .or(room::get_players(pool.clone(), key.clone()));

    let routes = login_routes.or(room_routes);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
