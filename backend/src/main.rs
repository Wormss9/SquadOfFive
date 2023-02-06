mod database;

use database::connect;

use deadpool_postgres::Pool;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

mod authorization;

mod rejection;

mod handlers;

use authorization::authorization;

use crate::authorization::Login;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Name {
    name: String,
}

fn main() {
    let pool = connect();

    let db_filter = warp::any().map(move || -> Pool { pool.clone() });

    fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: DeserializeOwned + Send,
    {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    let get_login = warp::get()
        .and(warp::path("api"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(db_filter.clone())
        .and_then(authorization::login)
        .recover(rejection::handle_rejection);

    let put_register = warp::put()
        .and(warp::path("api"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(db_filter.clone())
        .and_then(handlers::register)
        .recover(rejection::handle_rejection);

    let routes = put_register;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
        })
}
