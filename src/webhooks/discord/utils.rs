use crate::{pretty::log_error, webhooks::discord::config::DiscordConfig};
use std::process;

pub fn is_discord_activated (config: &Option<DiscordConfig>) -> bool {
  if config.is_none() {
    return false;
  }

  if let Some(inner) = config {
    if inner.webhook.is_none() {
      return false;
    }
  }

  return true;
}

pub fn convert_color (color: &str) -> u32 {
  u32::from_str_radix(color, 16).unwrap_or_else(|e| {
    log_error(e.to_string());
    process::exit(1);
  })
}