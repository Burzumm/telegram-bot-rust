use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    chat_id: i64,
    text: String,
}

struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
}

impl TelegramBot {
    fn new(api_token: String) -> Self {
        return TelegramBot {
            telegram_api_url: String::from("https://api.telegram.org/"),
            api_token,
        };
    }

    async fn send_message(&self, message: Message) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "{}{}{}{}",
                self.telegram_api_url, "bot", &self.api_token, "/sendMessage"
            ))
            .json(&message)
            .send()
            .await;
        return res;
    }
}
