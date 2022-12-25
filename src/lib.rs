use core::time;
use std::{thread, time::Duration};

use reqwest::{Error, Response};
use serde::{Deserialize, Serialize, de};

impl Message {
    pub fn new(chat_id: i64, text: String) -> Self {
        return Message { chat_id, text };
    }
}


impl TelegramBot {
    pub fn new(api_token: String) -> Self {
        let token = String::from(&api_token);
        return TelegramBot {
            telegram_api_url: String::from("https://api.telegram.org/"),
            api_token,
            telegram_bot_api_url: format!(
                "{}{}{}{}",
                "https://api.telegram.org/", "bot", token, "/"
            ),
        };
    }

    pub async fn get_me(&self) {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}{}", self.telegram_bot_api_url, "getMe"))
            .send()
            .await;
        println!("connect result: {}", res.unwrap().text().await.unwrap())
    }

    pub async fn send_message(&self, message: Message) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}{}", self.telegram_bot_api_url, "sendMessage"))
            .json(&message)
            .send()
            .await;
        return res;
    }

    pub async fn get_updates<TFunc, TResponseResult: de::DeserializeOwned>(
        &self,
        callback: fn(&TFunc, Vec<TResponseResult>),
        update_timeout_millis: u64,
        func_param: TFunc,
    ) {
        let client = reqwest::Client::new();
        loop {
            let res = client
                .get(format!("{}{}", self.telegram_bot_api_url, "getUpdates"))
                .timeout(Duration::from_millis(update_timeout_millis))
                .send()
                .await;
            let update_resault = res
                .unwrap()
                .json::<TelegramResponseResult<Vec<TResponseResult>>>()
                .await
                .unwrap();
            if update_resault.result.len() > 0 {
                callback(&func_param, update_resault.result);
            }
            thread::sleep(time::Duration::from_millis(1500));
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TelegramResponseResult<T> {
    ok: bool,
    result: T,
}

#[derive(Serialize, Deserialize)]
struct TelegramUpdate {
    update_id: i64,
    message: TelelgamMessage,
}

#[derive(Serialize, Deserialize)]
pub struct TelelgamMessage {
    message_id: i64,
    text: String,
    date: i64,
}

pub struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
    telegram_bot_api_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    chat_id: i64,
    text: String,
}