use std::env;

use postgres::{Client, NoTls, Row};

pub use player::Player;

mod player;

pub trait Database {
    fn from_row(row: Row) -> Self;
    fn create_table(client: &mut Client) -> Result<(), postgres::Error>;
}

fn initialize(client: &mut Client) -> Result<(), postgres::Error> {
    Player::create_table(client)
}

pub fn create_database_connection() -> Client {
    let user = env::var("POSTGRES_USER").expect("Missing POSTGRES_USER");
    let password = env::var("POSTGRES_PASSWORD").expect("Missing POSTGRES_PASSWORD");
    let dbname = env::var("POSTGRES_DB").expect("Missing POSTGRES_DB");
    let host = "localhost";
    let params = format!("user={user} password={password} dbname={dbname} host={host}");
    let mut client =
        Client::connect(&params, NoTls).expect(&format!("Failed to connect to {dbname}"));

    initialize(&mut client).expect("Failed to initialize db");

    client
}
