use crate::utils::error::Error;

pub use self::{game_user::GameUser, player::Player, room::Room};
use axum::async_trait;
use deadpool_postgres::{ManagerConfig, Object, Pool, RecyclingMethod, Runtime, Transaction};
use http::StatusCode;
use std::env;
use tokio_postgres::NoTls;

mod default_image;
pub mod game_user;
pub mod player;
pub mod room;

#[async_trait]
pub trait Database {
    async fn create_table(transaction: &Transaction<'_>) -> Result<(), Error>;
}

async fn initialize(pool: &Pool) -> Result<(), Error> {
    let mut client = initialize_client(pool).await?;
    let transaction = initialize_transaction(&mut client).await?;
    GameUser::create_table(&transaction).await?;
    Room::create_table(&transaction).await?;
    Player::create_table(&transaction).await?;
    commit(transaction).await
}

pub async fn initialize_client(pool: &Pool) -> Result<Object, Error> {
    pool.get()
        .await
        .map_err(Error::code_fn(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn initialize_transaction(object: &mut Object) -> Result<Transaction<'_>, Error> {
    object
        .transaction()
        .await
        .map_err(Error::code_fn(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn commit(transaction: Transaction<'_>) -> Result<(), Error> {
    transaction
        .commit()
        .await
        .map_err(Error::code_fn(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn get_pool() -> Pool {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some(env::var("POSTGRES_USER").expect("Missing POSTGRES_USER"));
    config.password = Some(env::var("POSTGRES_PASSWORD").expect("Missing POSTGRES_PASSWORD"));
    let dbname = env::var("POSTGRES_DB").expect("Missing POSTGRES_DB");
    config.dbname = Some(dbname.clone());
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let host = match env::var("DOCKERIZED") {
        Ok(_) => "db",
        Err(_) => "localhost",
    }
    .to_owned();

    config.host = Some(host);

    let pool = config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .unwrap_or_else(|_| panic!("Failed to connect to {dbname}"));
    initialize(&pool).await.expect("Failed to initialize db");
    pool
}
