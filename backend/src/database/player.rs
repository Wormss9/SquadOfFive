use postgres::{Row, Client};

use super::Database;

#[derive(Debug)]
pub struct Player {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub steam_id: Option<i32>,
    pub nick: String,
    pub avatar: String,
}

impl Database for Player {
    fn from_row(row: Row) -> Self {
        print!("{:?}", row);
        Self {
            id: row.get("id"),
            name: row.get("name"),
            password: row.get("password"),
            steam_id: row.get("steam_id"),
            nick: row.get("nick"),
            avatar: row.get("avatar"),
        }
    }
    fn create_table(client: &mut Client) -> Result<(), postgres::Error> {
        client.batch_execute(
            "CREATE TABLE IF NOT EXISTS player (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR UNIQUE NULLS NOT DISTINCT,
    password        VARCHAR,
    steam_id        INT UNIQUE NULLS NOT DISTINCT,
    nick            VARCHAR NOT NULL,
    avatar          VARCHAR NOT NULL DEFAULT ''
    );",
        )?;
        client.batch_execute("CREATE INDEX IF NOT EXISTS name_index ON player(name);")?;
        client.batch_execute("CREATE INDEX IF NOT EXISTS steam_index ON player(steam_id);")
    }
}


impl Player{
    pub fn create(
        client: &mut Client,
        name: &str,
        password: &str,
    ) -> Result<u64, postgres::Error> {
        client.execute(
            "INSERT INTO player (name, nick, password) VALUES ($1, $1, $2);",
            &[&name, &password],
        )
    }
    pub fn create_steam(
        client: &mut Client,
        steam_id: &str,
        nick: &str,
        avatar: &str,
    ) -> Result<u64, postgres::Error> {
        client.execute(
            "INSERT INTO player (steamId, nick,avatar) VALUES ($1, $2, $3)",
            &[&steam_id, &nick, &avatar],
        )
    }
    pub fn get(client: &mut Client, name: &str) -> Result<Option<Self>, postgres::Error> {
        let row = client.query_opt("SELECT * FROM player WHERE name = ($1);", &[&name])?;
    
        Ok(match row{
            Some(row) => Some(Player::from_row(row)),
            None => None,
        })
    }
    pub fn get_steam(client: &mut Client, steam_id: &str) -> Result<Option<Self>, postgres::Error> {
        let row = client.query_opt("SELECT * FROM player WHERE steamId = ($1);", &[&steam_id])?;
    
        Ok(match row{
            Some(row) => Some(Player::from_row(row)),
            None => None,
        })
    }
    
}