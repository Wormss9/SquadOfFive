use crate::{
    database::{initialize_client, initialize_transaction, GameUser, commit},
    utils::{
        authorization::{create_token, verify_password, Key, Login},
        error::Error,
    },
};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use deadpool_postgres::Pool;
use http::StatusCode;

#[debug_handler]
pub async fn user_login(
    State((pool, key)): State<(Pool, Key)>,
    Query(login): Query<Login>,
) -> Result<impl IntoResponse, Error> {
    let player = match match GameUser::get(&pool, &login.name).await {
        Ok(p) => p,
        Err(_) => return Err(Error::code(StatusCode::INTERNAL_SERVER_ERROR)),
    } {
        Some(p) => p,
        None => return Err(Error::code(StatusCode::NOT_FOUND)),
    };
    let player_password = match &player.password {
        Some(x) => x,
        None => return Err(Error::code(http::StatusCode::INTERNAL_SERVER_ERROR)),
    };
    if !verify_password(login.password, player_password) {
        return Err(Error::code(http::StatusCode::UNAUTHORIZED));
    }
    match create_token(player, key) {
        Ok(token) => Ok(axum::Json(token)),
        Err(_) => Err(Error::code(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
#[debug_handler]
pub async fn user_register(
    State((pool, _)): State<(Pool, Key)>,
    Json(login): Json<Login>,
) -> Result<impl IntoResponse, Error> {
    let mut client = initialize_client(&pool).await?;
    let transaction = initialize_transaction(&mut client).await?;

    GameUser::create(&transaction, &login.name, &login.password).await?;

    commit(transaction).await?;
    Ok((StatusCode::CREATED, "CREATED".to_owned()))
}
