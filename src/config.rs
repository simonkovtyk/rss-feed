use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rss {
  pub url: String,
  pub interval: u64,
  pub discord_webhook: Option<DiscordWebhook>
}

#[derive(Serialize, Deserialize)]
pub struct DiscordWebhook {
  pub id: u32,
  pub token: u32
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub db: String,
  pub rss: Vec<Rss>
}

pub fn get_config (path: &str) -> Config {
  let data = fs::read_to_string(path).expect("No config file present");

  return serde_json::from_str(&data).expect("Could not parse config");
}