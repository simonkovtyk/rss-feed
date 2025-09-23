use std::{future::{self, Pending}, process, time::Duration};
use reqwest::StatusCode;
use rss;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio::{task, time::{self, Instant}};

mod config;
mod env;
mod http;
mod models;
mod services;
mod utils;
mod webhooks;
mod crypt;
mod init;
mod pretty;

use crate::{config::get_config, http::get_rss, init::init_db, pretty::{get_header, log_error, log_info, log_warning}, services::{update_channel, update_posts}, webhooks::discord::{api::send_discord_webhook, utils::is_discord_activated}};

// seconds btw
const DEFAULT_INTERVAL: u64 = 3600;

#[tokio::main]
async fn main() -> () {
  println!("{}", get_header());
  log_info("Starting");

  let args = env::Args::parse();

  log_info("Parsed process arguments");

  let config_path = args.config;
  let config = get_config(&config_path);

  log_info("Parsed config");

  let pool = PgPoolOptions::new().max_connections(3).connect(&config.db).await.unwrap_or_else(|err| {
    log_error(err.to_string());
    process::exit(1);
  });

  init_db(&pool).await;

  log_info("Established connection to PostgreSQL");
  log_info("Started successfully, proceeding...");

  for rss in config.rss {
    let pool = pool.clone();
    task::spawn(async move {
      // Get initial sleep. It can be only the defined interval or the default interval
      let mut sleep = Duration::from_secs(
        rss
          .interval
          .unwrap_or(DEFAULT_INTERVAL)
      );
      let mut last_modified = None;

      loop {
        let start_time = Instant::now();
        let rss_response = get_rss(&rss.url, last_modified.clone()).await;

        if rss_response.is_err() {
          log_warning(format!("RSS-Feed with URL '{}' received a HTTP-Error while requesting the feed, continuing...", rss.url));
          time::sleep(sleep).await;
          continue;
        }

        let rss_response = rss_response.unwrap();

        // Save newest last modified header, so we can maybe detect changes next iteration
        if let Some(last_modified_header) = rss_response.headers().get("Last-Modified") {
          if let Ok(inner_last_modified) = last_modified_header.to_str() {
            last_modified = Some(inner_last_modified.to_string());
          } else {
            last_modified = None;
          }
        }

        // Don't need to go further since we already know its not modified or has errors (who is we btw?)
        if rss_response.status() == StatusCode::NOT_MODIFIED {
          time::sleep(sleep).await;
          continue;
        }

        if !rss_response.status().is_success() {
          log_warning(format!("RSS-Feed with URL '{}' received a unhandled HTTP-Error status while requesting the feed, continuing...", rss.url));
          time::sleep(sleep).await;
          continue;
        }

        let rss_content = rss_response.bytes().await;

        if rss_content.is_err() {
          log_warning(format!("RSS-Feed with URL '{}' doesn't provide any data, continuing...", rss.url));
          time::sleep(sleep).await;
          continue;
        }

        let rss_decode = rss::Channel::read_from(&rss_content.unwrap()[..]);

        if rss_decode.is_err() {
          log_warning(format!("RSS-Content of URL '{}' could not be parsed, continuing...", rss.url));
          time::sleep(sleep).await;
          continue;
        }

        let rss_channel = rss_decode.unwrap();
        let channel = update_channel(&pool, rss_channel.clone()).await;

        if channel.is_none() {
          log_warning("Could not insert Channel into DB, continuing...");
          time::sleep(sleep).await;
          continue;
        }

        let channel = channel.unwrap();

        if channel.id.is_none() {
          log_warning("Trying to insert a post with constraint of none, continuing...");
          time::sleep(sleep).await;
          continue;
        }

        let posts = update_posts(&pool, rss_channel.items, channel.id.unwrap()).await;

        if is_discord_activated(&rss.discord.clone()) {
          send_discord_webhook(&rss.discord.clone().unwrap(), &channel, &posts).await;
        }

        sleep = Duration::from_secs(
          rss
            .interval
            .unwrap_or(
              rss_channel
                .ttl
                .map(|ttl| ttl.parse().unwrap_or(DEFAULT_INTERVAL)
              ).unwrap_or(DEFAULT_INTERVAL)
            )
        );

        let duration = start_time.elapsed();
        let duration_diff = sleep.saturating_sub(duration);

        time::sleep(duration_diff).await;
      }
    });
  }

  let pending: Pending<()> = future::pending();

  pending.await;
}
