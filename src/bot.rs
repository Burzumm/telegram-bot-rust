use crate::TelegramBot;
use reqwest::Client;
use tracing::{error, info};
use tracing_attributes::instrument;

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

    #[instrument]
    pub async fn get_me(&self) {
        let client = Client::new();
        let res = client
            .get(format!("{}{}", self.telegram_bot_api_url, "getMe"))
            .send()
            .await;
        match res {
            Ok(result) => info!("connect result: {}", result.text().await.unwrap()),
            Err(error) => error!("connect result: {}", error),
        }
    }
}
