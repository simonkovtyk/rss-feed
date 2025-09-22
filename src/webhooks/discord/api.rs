use std::{process, time::Duration};
use reqwest::StatusCode;
use tokio::time::sleep;

use crate::{models::{Channel, Post}, pretty::{log_error, log_warning}, webhooks::discord::{config::DiscordConfig, mapping::map_to_discord_message, models::TooManyRequestsResponseBody}};

const DEFAULT_DELAY: u64 = 2000;

pub async fn send_discord_webhook (config: &DiscordConfig, channel: &Channel, posts: &Vec<Post>) {
  let webhook_config = config.clone().webhook.unwrap();
  let url = format!("https://discord.com/api/webhooks/{}/{}", webhook_config.id, webhook_config.token);
  let client = reqwest::Client::new();
  let delay = if let Some(delay) = webhook_config.delay {
    delay
  } else {
    DEFAULT_DELAY
  };

  for post in posts.iter() {
    let message = map_to_discord_message(config.clone(), channel, post);

    loop {
      let response = client.post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .send()
        .await;

      if response.is_err() {
        log_warning("Trying to get an empty response, continuing...");
        sleep(Duration::from_millis(delay)).await;
        return;
      }

      let response = response.unwrap();

      if !response.status().is_success() {
        log_warning("Received response of unknown error, continuing...");
        sleep(Duration::from_millis(delay)).await;
        continue;
      }

      match response.status() {
        StatusCode::TOO_MANY_REQUESTS => {
          let response_text = response.text().await;

          if response_text.is_err() {
            log_warning("Trying to read an empty response body, continuing...");
            sleep(Duration::from_millis(delay)).await;
            continue;
          }

          let body = TooManyRequestsResponseBody::try_from(response_text.unwrap());

          if body.is_err() {
            log_error("Could not parse response body, continuing...");
            process::exit(1);
          }

          sleep(Duration::from_secs_f32(body.unwrap().retry_after)).await;
        },
        _ => {}
      }

      break;
    }

    sleep(Duration::from_millis(delay)).await;
  }
}