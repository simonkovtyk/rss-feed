use sqlx::{Pool, Postgres};

use crate::{models::{Channel, Post}, utils::{map_channel, map_post}};

pub async fn update_channel (pool: &Pool<Postgres>, rss_channel: rss::Channel) -> Option<Channel> {
  let mut channel = map_channel(rss_channel);

  let result = sqlx::query_as::<_, Channel>("SELECT * FROM channels WHERE checksum = $1")
    .bind(&channel.checksum)
    .fetch_one(pool)
    .await
    .ok();

  if let Some(result_inner) = result {
    channel.id = result_inner.id;
    return Some(channel);
  }

  let row: Option<(i64,)> = sqlx::query_as("INSERT INTO channels (name, image_url, checksum) VALUES ($1, $2, $3) RETURNING id")
    .bind(&channel.name)
    .bind(&channel.image_url)
    .bind(&channel.checksum)
    .fetch_one(pool)
    .await
    .ok();

  if row.is_none() {
    return None;
  }

  channel.id = Some(row.unwrap().0);
  return Some(channel);
}

// Gets distinct items as side-effect
pub async fn update_posts (pool: &Pool<Postgres>, rss_items: Vec<rss::Item>, channel_id: i64) -> Vec<Post> {
  let mut posts = Vec::new();

  for rss_item in rss_items {
    //println!("{:?}", rss_item);
    let mapped = map_post(rss_item, channel_id);

    let result = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE checksum = $1")
      .bind(&mapped.checksum)
      .fetch_one(pool)
      .await;

    if result.is_ok() {
      continue;
    }

    sqlx::query_as::<_, Post>("INSERT INTO posts (title, link, pub_date, content, checksum, channel_id) VALUES ($1, $2, $3, $4, $5, $6)")
      .bind(&mapped.title)
      .bind(&mapped.link)
      .bind(&mapped.pub_date)
      .bind(&mapped.content)
      .bind(&mapped.checksum)
      .bind(&mapped.channel_id)
      .fetch_optional(pool)
      .await
      .expect("Could not insert Post");

    posts.push(mapped);
  }

  return posts;
}