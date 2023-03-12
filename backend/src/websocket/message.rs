use crate::{game_logic::play::Card, utils::error::Error};
use axum::extract::ws::Message;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyMessage {
    #[serde(rename = "type")]
    pub kind: String,
    pub message: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageType {
    Number(i32),
    Numbers(Vec<i32>),
    String(String),
    Vec(Vec<String>),
    Cards(Vec<Card>),
    NumbersNumbers((i32, i32)),
    Empty(Option<bool>),
}

impl MyMessage {
    pub fn as_string(&self) -> String {
        to_string(self).unwrap()
    }
    pub fn as_message(&self) -> Message {
        Message::Text(self.as_string())
    }
    pub fn joined(id: i32) -> Self {
        Self {
            kind: "joined".to_owned(),
            message: MessageType::Number(id),
        }
    }
    pub fn disconnect(id: i32) -> Self {
        Self {
            kind: "left".to_owned(),
            message: MessageType::Number(id),
        }
    }
    pub fn online(id: Vec<i32>) -> Self {
        Self {
            kind: "online".to_owned(),
            message: MessageType::Numbers(id),
        }
    }
    pub fn cards(id: Vec<Card>) -> Self {
        Self {
            kind: "cards".to_owned(),
            message: MessageType::Cards(id),
        }
    }
    pub fn table(play: Vec<Card>) -> Self {
        Self {
            kind: "table".to_owned(),
            message: MessageType::Cards(play),
        }
    }
    pub fn turn(turn: i32) -> Self {
        Self {
            kind: "turn".to_owned(),
            message: MessageType::Number(turn),
        }
    }
    pub fn card_amount(id: i32, amount: i32) -> Self {
        Self {
            kind: "card_amount".to_owned(),
            message: MessageType::NumbersNumbers((id, amount)),
        }
    }
    pub fn error<T: Debug>(message: T) -> Self {
        Self {
            kind: "error".to_owned(),
            message: MessageType::String(format!("{:?}", message)),
        }
    }
}

impl TryFrom<Message> for MyMessage {
    fn try_from(message: Message) -> Result<Self, Error> {
        let message = message
            .to_text()
            .map_err(Error::code_fn(StatusCode::BAD_REQUEST))?;
        serde_json::from_str(message).map_err(Error::message_fn(
            StatusCode::BAD_REQUEST,
            message.to_owned(),
        ))
    }
    type Error = Error;
}
