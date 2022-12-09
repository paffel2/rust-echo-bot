use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TgGetMeResult {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u64>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct TgUser {
    pub id: u64,
}
#[derive(Deserialize)]
pub struct TgMessage {
    pub message_id: u64,
    pub from: Option<TgUser>,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct TgChat {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct TgCallbackData {
    pub data: String,
    pub from: TgUser,
    pub message: TgMessage,
}

#[derive(Deserialize)]
pub struct TgUpdate {
    pub update_id: u64,
    pub message: Option<TgMessage>,
    pub callback_query: Option<TgCallbackData>,
}

pub enum MessageType {
    Help,
    Repeat,
    Simple,
}

#[derive(Deserialize, Serialize)]
pub struct Button {
    pub text: String,
    pub callback_data: String,
}
#[derive(Deserialize, Serialize)]
pub struct Keyboard {
    pub inline_keyboard: Vec<Vec<Button>>,
}
#[derive(Copy, Clone, Debug)]
pub enum Status {
    WaitNumber(u64),
    CurrentNumber(u8),
}
