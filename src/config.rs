use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested = true)]
    pub discord: DiscordConfig,
    #[envconfig(nested = true)]
    pub leetcode: LeetCodeConfig,
}

#[derive(Envconfig)]
pub struct DiscordConfig {
    #[envconfig(from = "DISCORD_API_ENDPOINT", default = "https://discord.com/api/channels")]
    pub api_endpoint: String,
    /// Bot auth token
    #[envconfig(from = "DISCORD_TOKEN")]
    pub token: String,
    /// Channel id where post message
    #[envconfig(from = "DISCORD_CHANNEL_ID")]
    pub channel_id: String,
}

#[derive(Envconfig)]
pub struct LeetCodeConfig {
    #[envconfig(from = "LEETCODE_API_ENDPOINT", default = "https://leetcode.com/graphql")]
    pub api_endpoint: String,
    /// Base url to leetcode to construct url to specific problem
    #[envconfig(from = "LEETCODE_BASE_URL", default = "https://leetcode.com")]
    pub base_url: String,
    /// Url to leetcode logo to construct discrod message with
    #[envconfig(from = "LEETCODE_IMAGE_URL", default = "https://leetcode.com/static/images/LeetCode_logo_rvs.png")]
    pub image_url: String,
}
