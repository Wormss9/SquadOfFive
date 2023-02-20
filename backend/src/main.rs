use authorization::get_key;
use database::connect;
use filters::login::{get_login, get_restricted, put_register};
use filters::room::{create_room, get_my_rooms, get_room, join_room};
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

    let login_routes = get_login(pool.clone(), key.clone())
        .or(put_register(pool.clone()))
        .or(get_restricted(key.clone()));

    let room_routes = create_room(pool.clone(), key.clone())
        .or(get_room(pool.clone(), key.clone()))
        .or(join_room(pool.clone(), key.clone()))
        .or(get_my_rooms(pool.clone(), key.clone()));

    let routes = login_routes.or(room_routes);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
