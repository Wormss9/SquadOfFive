use super::{initialize_client, Database, Player};
use crate::{game_logic::play::Card, utils::error::Error};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::Row;
use ulid::Ulid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Room {
    pub ulid: String,
    pub host: i32,
    pub play: Vec<Card>,
    pub turn: i32,
    pub last_turn: i32,
}

impl From<Row> for Room {
    fn from(row: Row) -> Self {
        Self {
            ulid: row.get("ulid"),
            host: row.get("host"),
            play: row.get("play"),
            turn: row.get("turn"),
            last_turn: row.get("last_turn"),
        }
    }
}

#[async_trait]
impl Database for Room {
    async fn create_table(pool: Pool) -> Result<(), Error> {
        let client = pool.get().await.expect("Failed to connect to db");
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS room (
                    ulid            VARCHAR(26) PRIMARY KEY,
                    host            INT REFERENCES game_user(id),
                    play            JSON[] DEFAULT array[]::JSON[],
                    turn            INT DEFAULT 0
                    last_turn            INT DEFAULT 0
                );",
            )
            .await
            .map_err(Error::from_db)
    }
}

impl Room {
    pub async fn create(pool: Pool, host: i32) -> Result<Room, Error> {
        let id = Ulid::new().to_string();
        let row = initialize_client(pool)
            .await?
            .query_one(
                "INSERT INTO room (ulid, host) VALUES ($1, $2) RETURNING *;",
                &[&id, &host],
            )
            .await
            .map_err(Error::from_db)?;
        Ok(Room::from(row))
    }
    pub async fn get(pool: Pool, id: &str) -> Result<Self, Error> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM room WHERE ulid = ($1);", &[&id])
            .await
            .map_err(Error::from_db)?;
        row.map(Room::from)
            .ok_or_else(|| Error::code(StatusCode::UNAUTHORIZED))
    }
    pub async fn get_my(pool: Pool, host: i32) -> Result<Vec<Self>, Error> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM room WHERE host = ($1);", &[&host])
            .await
            .map_err(Error::from_db)?;

        Ok(rows.into_iter().map(Room::from).collect())
    }
    pub async fn get_joined(pool: Pool, host: i32) -> Result<Vec<Self>, Error> {
        let rows = initialize_client(pool)
            .await?
            .query("select * from room where ulid in (select room from player where game_user = ($1)) and host != ($1);", &[&host])
            .await
            .map_err(Error::from_db)?;
        Ok(rows.into_iter().map(Room::from).collect())
    }
    pub async fn get_players(&self, pool: Pool) -> Result<Vec<Player>, Error> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM player WHERE room = ($1);", &[&self.ulid])
            .await
            .map_err(Error::from_db)?;

        Ok(rows.into_iter().map(Player::from).collect())
    }
    pub async fn update_turn(&self, pool: Pool, turn: i32) -> Result<(), Error> {
        initialize_client(pool)
            .await?
            .execute(
                "UPDATE room SET turn = $1 WHERE ulid = $2",
                &[&turn, &self.ulid],
            )
            .await
            .map_err(Error::from_db)?;
        Ok(())
    }
    pub async fn update_play(&self, pool: Pool, play: Vec<Card>) -> Result<(), Error> {
        initialize_client(pool)
            .await?
            .execute(
                "UPDATE room SET play = $1 WHERE ulid = $2",
                &[&play, &self.ulid],
            )
            .await
            .map_err(Error::from_db)?;
        Ok(())
    }
}
