use crate::{authorization::hash_password, rejection::get_internal_server_error};

use super::Database;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::{Error, Row};
use warp::Rejection;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub steam_id: Option<i32>,
    pub nick: String,
    pub avatar: String,
}

impl From<Row> for Player {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            password: row.get("password"),
            steam_id: row.get("steam_id"),
            nick: row.get("nick"),
            avatar: row.get("avatar"),
        }
    }
}

#[async_trait]
impl Database for Player {
    async fn create_table(pool: Pool) -> Result<(), Error> {
        let client = pool.get().await.expect("Failed to connect to db");
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS player (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR UNIQUE NULLS NOT DISTINCT,
    password        VARCHAR,
    steam_id        INT UNIQUE NULLS NOT DISTINCT,
    nick            VARCHAR NOT NULL,
    avatar          VARCHAR NOT NULL DEFAULT ''
    );",
            )
            .await?;
        client
            .batch_execute("CREATE INDEX IF NOT EXISTS name_index ON player(name);")
            .await?;
        client
            .batch_execute("CREATE INDEX IF NOT EXISTS steam_index ON player(steam_id);")
            .await
    }
}

impl Player {
    pub async fn create(pool: Pool, name: &str, password: &str) -> Result<u64, Rejection> {
        let client = pool
            .get()
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })?;
        client
            .execute(
                "INSERT INTO player (name, nick, password) VALUES ($1, $1, $2);",
                &[&name, &hash_password(password.to_owned())],
            )
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })
    }
    pub async fn create_steam(
        pool: Pool,
        steam_id: &str,
        nick: &str,
        avatar: &str,
    ) -> Result<u64, Rejection> {
        let client = pool
            .get()
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })?;
        client
            .execute(
                "INSERT INTO player (steamId, nick, avatar) VALUES ($1, $2, $3)",
                &[&steam_id, &nick, &avatar],
            )
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })
    }
    pub async fn get(pool: Pool, name: &str) -> Result<Option<Self>, Rejection> {
        let client = pool
            .get()
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })?;
        let row = client
            .query_opt("SELECT * FROM player WHERE name = ($1);", &[&name])
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })?;

        Ok(row.map(Player::from))
    }
    pub async fn get_steam(pool: Pool, steam_id: &str) -> Result<Option<Self>, Rejection> {
        let client = pool
            .get()
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })?;
        let row = client
            .query_opt("SELECT * FROM player WHERE steamId = ($1);", &[&steam_id])
            .await
            .map_err(|_| -> Rejection { get_internal_server_error() })?;

        Ok(row.map(Player::from))
    }
    pub fn get_identification(&self) -> PlayerIdentification {
        PlayerIdentification {
            id: self.id,
            name: self.name.clone(),
            steam_id: self.steam_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerIdentification {
    pub id: i32,
    pub name: Option<String>,
    pub steam_id: Option<i32>,
}
