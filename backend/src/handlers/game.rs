use crate::{
    database::game_user::UserIdentification,
    websocket::{self, WsPlayers},
};
use deadpool_postgres::Pool;
use warp::{ws::WebSocket, Reply};

pub fn join_game(
    room: String,
    user: UserIdentification,
    pool: Pool,
    players: WsPlayers,
    ws: warp::ws::Ws,
) -> impl Reply {
    ws.on_upgrade(move |socket| join(room, user, pool, players, socket))
}

async fn join(
    room: String,
    user: UserIdentification,
    pool: Pool,
    players: WsPlayers,
    socket: WebSocket,
) {
    let player = match match user.is_part_of_room(pool.clone(), &room).await {
        Ok(o) => o,
        Err(_) => return,
    } {
        Some(p) => p,
        None => return,
    };
    websocket::join(room, player, pool, players, socket).await
}
