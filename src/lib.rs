mod bot;
mod commands;
mod messages;
mod updates;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub text: Option<String>,
    pub date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: TelegramMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotCommand {
    command: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TelegramResponseResult<T> {
    ok: bool,
    result: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
    telegram_bot_api_url: String,
    last_update_id: i64,
}
