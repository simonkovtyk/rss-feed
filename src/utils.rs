use std::usize;

use serde::Serialize;
use sqlx::Encode;

use crate::models::{Channel, Post};

pub fn map_channel (channel: rss::Channel) -> Channel {
  let name = channel.title;
  let hash = blake3::hash(name.as_bytes()).to_string();

  return Channel {
    id: None,
    name,
    checksum: hash
  }
}

pub fn map_post (item: rss::Item, channel_id: i64) -> Post {
  let mut content = if let Some(item_content) = item.content {
    let text = html2text::from_read(item_content.as_bytes(), usize::MAX);

    if let Ok(inner_text) = text && inner_text != "None\n" {
      Some(inner_text)
    } else {
      None
    }
  } else {
    None
  };

  let mut post = Post {
    title: item.title,
    link: item.link,
    pub_date: item.pub_date,
    content,
    channel_id,
    checksum: "".to_string()
  };

  let mut post_bytes = bincode::encode_to_vec(&post, bincode::config::standard()).unwrap();

  let hash = blake3::Hasher::new().update(&post_bytes).finalize();

  post.checksum = hash.to_string();

  return post;
}