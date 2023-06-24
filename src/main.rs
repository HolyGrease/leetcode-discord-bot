mod configuration;
mod discord;
mod leetcode;

use crate::discord::DiscordApi;
use crate::leetcode::LeetCodeApi;
use config::{Config, File};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::builder()
        .add_source(File::with_name("config.yaml"))
        .build()?
        .try_deserialize::<crate::configuration::Config>()?;

    let leetcode_api = LeetCodeApi::new(&config.leetcode.api_endpoint);
    // Fetch daily problem from LeetCode
    let leetcode_problem = leetcode_api.fetch_daily_problem().await?;
    // Fetch problem description
    let description = leetcode_api
        .fetch_problem_description(&leetcode_problem.properties.title_slug)
        .await?;

    // Convert from HTML to markdown formatting
    let description = html2md::parse_html(&description);
    // Compose message
    let message = json!({
        "embed": {
            "title": leetcode_problem.properties.title,
            "description": description,
            "url": format!("{}{}", config.leetcode.base_url, leetcode_problem.link),
            "footer": {
                "text": format!("Difficulty: {}", leetcode_problem.properties.difficulty),
            },
            "thumbnail": {
                "url": config.leetcode.image_url
            }
        }
    });

    // Post message to discord channel
    DiscordApi::new(&config.discord.api_endpoint, &config.discord.token)
        .post_message(&config.discord.channel_id, message)
        .await?;

    Ok(())
}
