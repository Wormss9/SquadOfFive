use crate::{authorization::hash_password, rejection::MyRejection};

use super::{initialize_client, Database};
use crate::database::default_image::IMAGE;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::{Error, Row};
use warp::Rejection;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameUser {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub steam_id: Option<i32>,
    pub nick: String,
    pub avatar: String,
}

impl From<Row> for GameUser {
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
impl Database for GameUser {
    async fn create_table(pool: Pool) -> Result<(), Error> {
        let client = pool.get().await.expect("Failed to connect to db");
        let create_query = format!(
            "CREATE TABLE IF NOT EXISTS game_user (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR UNIQUE,
                    password        VARCHAR,
                    steam_id        INT UNIQUE,
                    nick            VARCHAR NOT NULL,
                    avatar          VARCHAR NOT NULL DEFAULT '{}');
            CREATE INDEX IF NOT EXISTS name_index ON game_user(name);
            CREATE INDEX IF NOT EXISTS steam_index ON game_user(steam_id);",
            IMAGE
        );
        client.batch_execute(&create_query).await
    }
}

impl GameUser {
    pub async fn create(pool: Pool, name: &str, password: &str) -> Result<u64, Rejection> {
        initialize_client(pool)
            .await?
            .execute(
                "INSERT INTO game_user (name, nick, password) VALUES ($1, $1, $2);",
                &[&name, &hash_password(password.to_owned())],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))
    }
    pub async fn create_steam(
        pool: Pool,
        steam_id: &str,
        nick: &str,
        avatar: &str,
    ) -> Result<u64, Rejection> {
        initialize_client(pool)
            .await?
            .execute(
                "INSERT INTO game_user (steamId, nick, avatar) VALUES ($1, $2, $3)",
                &[&steam_id, &nick, &avatar],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))
    }
    pub async fn get(pool: Pool, name: &str) -> Result<Option<Self>, Rejection> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM game_user WHERE name = ($1);", &[&name])
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(row.map(GameUser::from))
    }
    pub async fn get_steam(pool: Pool, steam_id: &str) -> Result<Option<Self>, Rejection> {
        let row = initialize_client(pool)
            .await?
            .query_opt(
                "SELECT * FROM game_user WHERE steamId = ($1);",
                &[&steam_id],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(row.map(GameUser::from))
    }
    pub fn get_identification(&self) -> UserIdentification {
        UserIdentification {
            id: self.id,
            name: self.name.clone(),
            steam_id: self.steam_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserIdentification {
    pub id: i32,
    pub name: Option<String>,
    pub steam_id: Option<i32>,
}
