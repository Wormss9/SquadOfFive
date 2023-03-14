use crate::{
    database::{game_user::UserIdentification, player, Room},
    websocket::{self, WsPlayers},
};
use axum::{
    debug_handler,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use deadpool_postgres::Pool;

use super::authorization::WsUserIdentification;

#[debug_handler]
pub async fn game(
    user: WsUserIdentification,
    State((pool, players)): State<(Pool, WsPlayers)>,
    ws: WebSocketUpgrade,
    Path(room): Path<String>,
) -> impl IntoResponse {
    let user = user.inner();
    ws.on_upgrade(move |socket| join(room, user, pool, players, socket))
}

pub async fn join(
    room: String,
    user: UserIdentification,
    pool: Pool,
    players: WsPlayers,
    mut socket: WebSocket,
) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_err() {
        return;
    }
    let room = match Room::get(&pool, &room).await {
        Ok(r) => r,
        Err(_) => return,
    };
    let r_players = match room.get_players(&pool).await {
        Ok(r) => r,
        Err(_) => return,
    };
    for player in r_players {
        if player.points >= 100 {
            return;
        }
    }
    let player = match user.is_part_of(&pool, &room).await {
        Ok(r) => r,
        Err(_) => return,
    };
    websocket::join(&room, &player, &pool, &players, socket).await
}
