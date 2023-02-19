use authorization::get_key;
use database::connect;
use filters::*;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

mod authorization;
mod database;
mod filters;
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

    let routes = get_login(pool.clone(), key.clone())
        .or(put_register(pool.clone()))
        .or(get_restricted(key.clone()));

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
