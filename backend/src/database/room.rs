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
    pub ended: bool,
}

impl From<Row> for Room {
    fn from(row: Row) -> Self {
        Self {
            ulid: row.get("ulid"),
            host: row.get("host"),
            play: row.get("play"),
            turn: row.get("turn"),
            last_turn: row.get("last_turn"),
            ended: row.get("ended"),
        }
    }
}

#[async_trait]
impl Database for Room {
    async fn create_table(pool: &Pool) -> Result<(), Error> {
        let client = pool.get().await.expect("Failed to connect to db");
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS room (
                    ulid            VARCHAR(26) PRIMARY KEY,
                    host            INT REFERENCES game_user(id) ON delete cascade,
                    play            JSON[] DEFAULT array[]::JSON[],
                    turn            INT DEFAULT 0,
                    last_turn       INT DEFAULT 0,
                    ended           BOOLEAN DEFAULT FALSE
                );",
            )
            .await
            .map_err(Error::from_db)
    }
}

impl Room {
    pub async fn create(pool: &Pool, host_id: i32) -> Result<Room, Error> {
        let ulid = Ulid::new().to_string();
        let row = initialize_client(pool)
            .await?
            .query_one(
                "INSERT INTO room (ulid, host) VALUES ($1, $2) RETURNING *;",
                &[&ulid, &host_id],
            )
            .await
            .map_err(Error::from_db)?;
        Ok(Room::from(row))
    }
    pub async fn get(pool: &Pool, ulid: &str) -> Result<Self, Error> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM room WHERE ulid = ($1);", &[&ulid])
            .await
            .map_err(Error::from_db)?;
        row.map(Room::from)
            .ok_or_else(|| Error::code(StatusCode::UNAUTHORIZED))
    }
    pub async fn get_owned(pool: &Pool, host_id: i32) -> Result<Vec<Self>, Error> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM room WHERE host = ($1);", &[&host_id])
            .await
            .map_err(Error::from_db)?;

        Ok(rows.into_iter().map(Room::from).collect())
    }
    pub async fn get_joined(pool: &Pool, host_id: i32) -> Result<Vec<Self>, Error> {
        let rows = initialize_client(pool)
            .await?
            .query("select * from room where ulid in (select room from player where game_user = ($1)) and host != ($1);", &[&host_id])
            .await
            .map_err(Error::from_db)?;
        Ok(rows.into_iter().map(Room::from).collect())
    }
    pub async fn get_players(&self, pool: &Pool) -> Result<Vec<Player>, Error> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM player WHERE room = ($1);", &[&self.ulid])
            .await
            .map_err(Error::from_db)?;

        Ok(rows.into_iter().map(Player::from).collect())
    }
    pub async fn update(&self, pool: &Pool) -> Result<(), Error> {
        let row = initialize_client(pool)
            .await?
            .execute(
                "UPDATE room SET play = $1, turn = $2, last_turn = $3, ended = $4 WHERE ulid = $5",
                &[
                    &self.play,
                    &self.turn,
                    &self.last_turn,
                    &self.ended,
                    &self.ulid,
                ],
            )
            .await
            .map_err(Error::from_db)?;
        if row != 1 {
            return Err(Error::message(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Updated {} rows", row),
            ));
        }
        Ok(())
    }
    pub async fn delete(&self, pool: &Pool) -> Result<(), Error> {
        let row = initialize_client(pool)
            .await?
            .execute("DELETE FROM room WHERE ulid = $1", &[&self.ulid])
            .await
            .map_err(Error::from_db)?;
        if row != 1 {
            return Err(Error::message(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Updated {} rows", row),
            ));
        }
        Ok(())
    }
    pub fn increment_turn(&mut self) -> &mut Self {
        self.turn = if self.turn == 3 { 0 } else { self.turn + 1 };
        self
    }
}
