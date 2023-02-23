use super::authorization::{create_token, verify_password, Login};
use crate::database::game_user::UserIdentification;
use crate::database::GameUser;
use crate::filters::rejection::MyRejection;
use deadpool_postgres::Pool;
use hmac::Hmac;
use http::Response;
use http::StatusCode;
use sha2::Sha512;
use warp::Rejection;
use warp::Reply;

pub async fn user_login(
    login: Login,
    pool: Pool,
    key: Hmac<Sha512>,
) -> Result<impl Reply, Rejection> {
    let player = match match GameUser::get(pool, &login.name).await {
        Ok(p) => p,
        Err(_) => return Err(MyRejection::message(StatusCode::INTERNAL_SERVER_ERROR, "1")),
    } {
        Some(p) => p,
        None => return Err(MyRejection::code(StatusCode::NOT_FOUND)),
    };
    let player_password = match &player.password {
        Some(x) => x,
        None => {
            return Err(MyRejection::message(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "2",
            ))
        }
    };
    if !verify_password(login.password, player_password) {
        return Err(MyRejection::code(http::StatusCode::BAD_REQUEST));
    }
    match create_token(player, key) {
        Ok(token) => Ok(
            Response::builder().body(serde_json::to_string(&token).unwrap())
    ),
        Err(_) => Err(MyRejection::message(StatusCode::INTERNAL_SERVER_ERROR, "3")),
    }
}

pub async fn user_register(login: Login, pool: Pool) -> Result<impl Reply, Rejection> {
    GameUser::create(pool.clone(), &login.name, &login.password).await?;
    Ok(warp::reply::with_status(
        "CREATED".to_owned(),
        StatusCode::CREATED,
    ))
}

pub async fn restricted_handler(_player: UserIdentification) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status(
        "Authorized".to_owned(),
        StatusCode::OK,
    ))
}
