use core::time;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};

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
            last_update_id: 0,
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

    pub async fn get_updates(&mut self, update_timeout_secs: u64) -> Vec<TelegramUpdate> {
        let client = reqwest::Client::new();

        let res = client
            .get(format!(
                "{}{}{}{}{}{}",
                self.telegram_bot_api_url,
                "getUpdates?",
                "offset=",
                &self.last_update_id + 1,
                "&timeout=",
                update_timeout_secs
            ))
            .timeout(Duration::from_secs(update_timeout_secs + 10))
            .send()
            .await;
        let update_resault = res
            .unwrap()
            .json::<TelegramResponseResult<Vec<TelegramUpdate>>>()
            .await
            .unwrap();
        if update_resault.result.len() > 0 {
            return update_resault.result;
        } else {
            return vec![];
        }
    }

    pub async fn update_callback<TFunc, TTelegramUpdate>(
        &mut self,
        callback: fn(&TFunc, Vec<TelegramUpdate>),
        update_timeout_secs: u64,
        func_param: TFunc,
    ) {
        let client = reqwest::Client::new();

        let res = client
            .get(format!(
                "{}{}{}{}{}{}",
                self.telegram_bot_api_url,
                "getUpdates?",
                "offset=",
                &self.last_update_id + 1,
                "&timeout=",
                update_timeout_secs
            ))
            .timeout(Duration::from_secs(update_timeout_secs + 10))
            .send()
            .await;
        let update_resault = res
            .unwrap()
            .json::<TelegramResponseResult<Vec<TelegramUpdate>>>()
            .await
            .unwrap();
        if update_resault.result.len() > 0 {
            for i in &update_resault.result {
                if i.update_id > self.last_update_id {
                    self.last_update_id = i.update_id;
                }
            }
            callback(&func_param, update_resault.result);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TelegramResponseResult<T> {
    ok: bool,
    result: T,
}

#[derive(Serialize, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: TelelgamMessage,
}

#[derive(Serialize, Deserialize)]
pub struct TelelgamMessage {
    pub message_id: i64,
    pub text: String,
    pub date: i64,
}

pub struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
    telegram_bot_api_url: String,
    last_update_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    chat_id: i64,
    text: String,
}
