use crate::TelegramBot;
use reqwest::Client;

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
        let client = Client::new();
        let res = client
            .get(format!("{}{}", self.telegram_bot_api_url, "getMe"))
            .send()
            .await;
        println!("connect result: {}", res.unwrap().text().await.unwrap())
    }
}
