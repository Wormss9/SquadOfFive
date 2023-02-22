use http::StatusCode;
use std::{convert::Infallible, error::Error};
use warp::{
    body::BodyDeserializeError,
    reject::Reject,
    reply::{self, WithStatus},
    Rejection, Reply,
};

#[derive(Debug)]
pub struct MyRejection {
    code: StatusCode,
    message: Option<String>,
}

impl Reject for MyRejection {}
impl MyRejection {
    fn get_reply(&self) -> WithStatus<String> {
        reply::with_status(self.message.clone().unwrap_or_default(), self.code)
    }
    pub fn code(code: StatusCode) -> Rejection {
        warp::reject::custom(Self {
            code,
            message: None,
        })
    }
    pub fn code_fn<T: std::fmt::Debug>(code: StatusCode) -> impl Fn(T) -> Rejection {
        move |x| -> Rejection {
            panic!("{:?}", x);
            Self::code(code)
        }
    }
    pub fn message(code: StatusCode, message: &str) -> Rejection {
        warp::reject::custom(Self {
            code,
            message: Some(message.to_owned()),
        })
    }
}

pub async fn handle_rejection(result: Rejection) -> Result<impl Reply, Infallible> {
    Ok(if result.is_not_found() {
        reply::with_status("NOT_FOUND".to_owned(), StatusCode::NOT_FOUND)
    } else if let Some(e) = result.find::<MyRejection>() {
        e.get_reply()
    } else if let Some(e) = result.find::<BodyDeserializeError>() {
        reply::with_status(
            match e.source() {
                Some(x) => x.to_string(),
                None => "BAD_REQUEST".to_owned(),
            },
            StatusCode::BAD_REQUEST,
        )
    } else {
        eprintln!("unhandled rejection: {:?}", result);
        reply::with_status(
            "INTERNAL_SERVER_ERROR".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}
