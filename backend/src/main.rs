mod database;

use database::connect;

use deadpool_postgres::Pool;
use serde::{de::DeserializeOwned};
use serde_derive::{Deserialize, Serialize};
use warp::{reply::WithStatus, Filter, Rejection};

use crate::database::Player;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Name {
    name: String,
}

fn main() {
    let pool = connect();

    let db_filter = warp::any().map(move || -> Pool { pool.clone() });

    // let db_filter = warp::any().map(move || -> Arc<RwLock<Pool>> { client.clone() });

    // let store = Store::new();
    // let store_filter = warp::any().map(move || store.clone());

    fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: DeserializeOwned + Send,
    {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
    fn authorization() -> impl Filter<Extract = (Option<Player>,), Error = warp::Rejection> + Clone {
        warp::header::header::<String>("Authorization").map(|x| -> Option<Player> {
            Some(Player {
                avatar: "".to_owned(),
                nick: "".to_owned(),
                id: 0,
                name: None,
                password: None,
                steam_id: None,
            })
        })
    }

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(authorization())
        .and(json_body())
        .and(db_filter.clone())
        .and_then(test);

    // let get_items = warp::post()
    //     .and(warp::path("v2"))
    //     .and(warp::path("groceries"))
    //     .and(warp::path::end())
    //     .and(json_body())
    //     .and(db_filter)
    //     .and_then(test);
    // let x = [add_items,get_items];
    let routes = add_items;
    // .or(get_items);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
        })
}

async fn test(player: Option<Player>, name: Name, pool: Pool) -> Result<WithStatus<String>, Rejection> {
    let x = Player::get(pool, &name.name).await.unwrap();
    if x.is_some() {
        Ok(warp::reply::with_status(
            format!("{:?}\n{:?}", x.unwrap(), player),
            http::StatusCode::CREATED,
        ))
    } else {
        Err(warp::reject::not_found())
    }
}
