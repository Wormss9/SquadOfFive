use std::env;

use async_trait::async_trait;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime};

pub use player::Player;
use tokio_postgres::{Error, NoTls};

mod player;

#[async_trait]
pub trait Database {
    async fn create_table(pool: Pool) -> Result<(), Error>;
}

async fn initialize(pool: Pool) -> Result<(), Error> {
    Player::create_table(pool).await
}

pub async fn connect() -> Pool {
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
        .expect(&format!("Failed to connect to {dbname}"));
    initialize(pool.clone()).await.expect("Failed to initialize db");
    pool
}
