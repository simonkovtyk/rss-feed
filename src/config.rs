use std::{fs, process};

use serde::{Deserialize, Serialize};

use crate::{pretty::log_error, webhooks::discord::config::DiscordConfig};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rss {
  pub url: String,
  pub interval: u64,
  pub discord: Option<DiscordConfig>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub db: String,
  pub rss: Vec<Rss>
}

pub fn get_config (path: &str) -> Config {
  let data = fs::read_to_string(path).unwrap_or_else(|err| {
    log_error(err.to_string());
    process::exit(1);
  });

  return serde_json::from_str(&data).unwrap_or_else(|err| {
    log_error(err.to_string());
    process::exit(1)
  });
}