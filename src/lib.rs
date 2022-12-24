use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    chat_id: i64,
    text: String,
}

pub struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
    telegram_bot_api_url: String,
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

    pub async fn get_updates<T>(
        &self,
        callback: fn(&T),
        update_timeout_millis: u64,
        func_param: T,
    ) {
        let client = reqwest::Client::new();
        loop {
            let res = client
            .get(format!(
                "{}{}",
                self.telegram_bot_api_url, "getUpdates"
            ))
            .send()
            .await;
            callback(&func_param);
            thread::sleep(time::Duration::from_millis(update_timeout_millis));
        }
    }
}
