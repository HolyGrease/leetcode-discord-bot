use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub leetcode: LeetCodeConfig,
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub api_endpoint: String,
    /// Bot auth token
    pub token: String,
    /// Channel id where post message
    pub channel_id: String,
}

#[derive(Deserialize)]
pub struct LeetCodeConfig {
    pub api_endpoint: String,
    /// Base url to leetcode to construct url to specific problem
    pub base_url: String,
    /// Url to leetcode logo to construct discrod message with
    pub image_url: String,
}
