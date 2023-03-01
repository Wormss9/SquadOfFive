use crate::utils::error::Error;

pub use self::{game_user::GameUser, player::Player, room::Room};
use axum::async_trait;
use deadpool_postgres::{ManagerConfig, Object, Pool, RecyclingMethod, Runtime};
use http::StatusCode;
use std::env;
use tokio_postgres::NoTls;

mod default_image;
pub mod game_user;
pub mod player;
pub mod room;

#[async_trait]
pub trait Database {
    async fn create_table(pool: Pool) -> Result<(), Error>;
}

async fn initialize(pool: Pool) -> Result<(), Error> {
    GameUser::create_table(pool.clone()).await?;
    Room::create_table(pool.clone()).await?;
    Player::create_table(pool).await
}

async fn initialize_client(pool: Pool) -> Result<Object, Error> {
    pool.get()
        .await
        .map_err(Error::code_fn(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn get_pool() -> Pool {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some(env::var("POSTGRES_USER").expect("Missing POSTGRES_USER"));
    config.password = Some(env::var("POSTGRES_PASSWORD").expect("Missing POSTGRES_PASSWORD"));
    let dbname = env::var("POSTGRES_DB").expect("Missing POSTGRES_DB");
    config.dbname = Some(dbname.clone());
    config.host = Some("localhost".to_owned());
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .unwrap_or_else(|_| panic!("Failed to connect to {dbname}"));
    initialize(pool.clone())
        .await
        .expect("Failed to initialize db");
    pool
}
