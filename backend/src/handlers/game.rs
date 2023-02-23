use crate::{
    database::{game_user::UserIdentification, Room},
    websocket::{self, WsPlayers},
};
use deadpool_postgres::Pool;
use warp::{ws::WebSocket, Reply};

pub fn game(
    room: String,
    user: UserIdentification,
    pool: Pool,
    players: WsPlayers,
    ws: warp::ws::Ws,
) -> impl Reply {
    ws.on_upgrade(move |socket| join(room, user, pool, players, socket))
}

pub async fn join(
    room: String,
    user: UserIdentification,
    pool: Pool,
    players: WsPlayers,
    socket: WebSocket,
) {
    let room = match Room::get(pool.clone(), &room).await {
        Ok(r) => r,
        Err(_) => return,
    };
    let player = match user.is_part_of(pool.clone(), &room).await {
        Ok(r) => r,
        Err(_) => return,
    };
    websocket::join(room, player, pool, players, socket).await
}
