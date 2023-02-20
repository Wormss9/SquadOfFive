use crate::database::game_user::UserIdentification;
use crate::rejection::MyRejection;
use crate::{
    authorization::{create_token, verify_password, Login},
    database::GameUser,
};
use deadpool_postgres::Pool;
use hmac::Hmac;
use http::StatusCode;
use sha2::Sha512;
use warp::reply::WithHeader;
use warp::{reply::WithStatus, Rejection};

pub async fn login(
    login: Login,
    pool: Pool,
    key: Hmac<Sha512>,
) -> Result<WithHeader<String>, Rejection> {
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
        Ok(token) => Ok(warp::reply::with_header(
            token.clone(),
            "set-cookie",
            format!("token={}; Path=/; HttpOnly; Max-Age=1209600", token),
        )),
        Err(_) => Err(MyRejection::message(StatusCode::INTERNAL_SERVER_ERROR, "3")),
    }
}

pub async fn register(login: Login, pool: Pool) -> Result<WithStatus<String>, Rejection> {
    GameUser::create(pool.clone(), &login.name, &login.password).await?;
    Ok(warp::reply::with_status(
        "CREATED".to_owned(),
        StatusCode::CREATED,
    ))
}

pub async fn restricted_handler(
    _player: UserIdentification,
) -> Result<WithStatus<String>, Rejection> {
    Ok(warp::reply::with_status(
        "Authorized".to_owned(),
        StatusCode::OK,
    ))
}
