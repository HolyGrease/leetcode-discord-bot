use reqwest::Client;
use serde::Serialize;

pub struct DiscordApi<'a> {
    api_endpoint: &'a str,
    token: &'a str,
}

impl<'a> DiscordApi<'a> {
    pub fn new(api_endpoint: &'a str, token: &'a str) -> Self {
        Self {
            api_endpoint,
            token,
        }
    }

    /// Post message to discord channel via HTTP
    ///
    /// # Errors
    ///
    /// if failed to connect to Discord's endpoint or
    /// failed to execute request
    pub async fn post_message<T: Serialize>(
        &self,
        channel_id: &str,
        message: T,
    ) -> anyhow::Result<()> {
        Client::builder()
            .build()?
            .post(format!("{}/{}/messages", self.api_endpoint, channel_id,))
            .header("Authorization", format!("Bot {}", self.token))
            .json(&message)
            .send()
            .await?;

        Ok(())
    }
}
