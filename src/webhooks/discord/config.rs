use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordWebhook {
  pub id: u128,
  pub token: String,
  pub delay: Option<u64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordStyle {
  pub embed: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordConfig {
  pub webhook: Option<DiscordWebhook>,
  pub style: Option<DiscordStyle>
}