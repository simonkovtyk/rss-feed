use std::{future::{self, Pending}, time::Duration};
use mongodb::Client;
use rss;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio::{task, time};
use anyhow::{Result, Error};

use crate::{config::get_config, http::get_rss, services::{update_channel, update_posts}};

mod config;
mod env;
mod http;
mod db;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> () {
  let args = env::Args::parse();
  let config_path = args.config;
  let config = get_config(&config_path);

  println!("{}", &config.db);
  let pool = PgPoolOptions::new().max_connections(3).connect(&config.db).await.expect("Could not connect to db.");

  for rss in config.rss {
    if rss.discord_webhook.is_none() {
      continue;
    }

    let pool = pool.clone();

    task::spawn(async move {
     
      let sleep = Duration::from_millis(rss.interval);

      loop {
        let rss_content = get_rss(&rss.url).await;
        
        if rss_content.is_err() {
          time::sleep(sleep).await;
          continue;
        }

        let rss_decode = rss::Channel::read_from(&rss_content.unwrap()[..]);

        if rss_decode.is_err() {
          time::sleep(sleep).await;
          continue;
        }

        let rss_channel = rss_decode.unwrap();

        let channel = update_channel(&pool, rss_channel.clone()).await;
        update_posts(&pool, rss_channel.items, channel.id.expect("Expect to have at least a primary key constraint")).await;
        time::sleep(sleep).await;
      }
    });
  }

  let pending: Pending<()> = future::pending();

  pending.await;
}
