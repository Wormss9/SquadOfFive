use super::{default_image::IMAGE, initialize_client, Database, Player, Room};
use crate::utils::{authorization::hash_password, error::Error};
use async_trait::async_trait;
use deadpool_postgres::{Pool, Transaction};
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::Row;

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
    async fn create_table(transaction: &Transaction<'_>) -> Result<(), Error> {
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
        transaction
            .batch_execute(&create_query)
            .await
            .map_err(Error::from_db)
    }
}

impl GameUser {
    pub async fn create(
        transaction: &Transaction<'_>,
        name: &str,
        password: &str,
    ) -> Result<u64, Error> {
        transaction
            .execute(
                "INSERT INTO game_user (name, nick, password) VALUES ($1, $1, $2);",
                &[&name, &hash_password(password.to_owned())],
            )
            .await
            .map_err(Error::from_db)
    }
    // pub async fn create_steam(
    //     pool: Pool,
    //     steam_id: &str,
    //     nick: &str,
    //     avatar: &str,
    // ) -> Result<u64, Error> {
    //     initialize_client(pool)
    //         .await?
    //         .execute(
    //             "INSERT INTO game_user (steamId, nick, avatar) VALUES ($1, $2, $3)",
    //             &[&steam_id, &nick, &avatar],
    //         )
    //         .await
    //         .map_err(Error::from_db)
    // }
    pub async fn get(pool: &Pool, name: &str) -> Result<Option<Self>, Error> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM game_user WHERE name = ($1);", &[&name])
            .await
            .map_err(Error::from_db)?;

        Ok(row.map(GameUser::from))
    }
    // pub async fn get_steam(pool: Pool, steam_id: &str) -> Result<Option<Self>, Error> {
    //     let row = initialize_client(pool)
    //         .await?
    //         .query_opt(
    //             "SELECT * FROM game_user WHERE steamId = ($1);",
    //             &[&steam_id],
    //         )
    //         .await
    //         .map_err(Error::from_db)?;
    //     Ok(row.map(GameUser::from))
    // }
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Self, Error> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM game_user WHERE id = ($1);", &[&id])
            .await
            .map_err(Error::from_db)?
            .ok_or_else(|| Error::code(StatusCode::NOT_FOUND))?;

        Ok(GameUser::from(row))
    }
    pub fn get_identification(&self) -> UserIdentification {
        UserIdentification {
            id: self.id,
            name: self.name.clone(),
            steam_id: self.steam_id,
        }
    }
    pub fn get_public(self) -> PublicUser {
        PublicUser {
            id: self.id,
            nick: self.nick,
            avatar: self.avatar,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserIdentification {
    pub id: i32,
    pub name: Option<String>,
    pub steam_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PublicUser {
    pub id: i32,
    pub nick: String,
    pub avatar: String,
}

impl UserIdentification {
    pub async fn is_part_of(&self, pool: &Pool, room: &Room) -> Result<Player, Error> {
        let players = room.get_players(pool).await?;
        players
            .into_iter()
            .find(|player| player.game_user == Some(self.id))
            .ok_or_else(|| Error::code(StatusCode::UNAUTHORIZED))
    }
    pub async fn get_game_user(self, pool: &Pool) -> Result<GameUser, Error> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM game_user WHERE id = ($1);", &[&self.id])
            .await
            .map_err(Error::from_db)?
            .ok_or_else(|| Error::code(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(GameUser::from(row))
    }
}
