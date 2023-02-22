use database::connect;
use filters::{game, room, signing};
use handlers::authorization::get_key;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;
use websocket::WsPlayers;

mod database;
mod filters;
mod game_logic;
mod handlers;
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

    let routes = signing::signing(db_pool.clone(), key.clone())
        .or(room::room(db_pool.clone(), key.clone()))
        .or(game::game(db_pool.clone(), key.clone(), rooms));

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
