use std::usize;
use chrono::DateTime;

use crate::{crypt::get_hash, models::{Channel, Post}};

pub fn map_channel (channel: rss::Channel) -> Channel {
  let name = channel.title;
  let hash = get_hash(&name);

  return Channel {
    id: None,
    image_url: if let Some(image) = channel.image {
      Some(image.url)
    } else {
      None
    },
    name,
    checksum: hash
  }
}

pub fn map_post (item: rss::Item, channel_id: i64) -> Post {
  let content = if let Some(item_content) = item.content {
    println!("---------");
    println!("{}", item_content);

    let text = html2text::from_read(item_content.as_bytes(), usize::MAX);

    if let Ok(inner_text) = text && inner_text != "None\n" {
      Some(inner_text)
    } else {
      None
    }
  } else {
    None
  };

      println!("---------");


  let mut post = Post {
    title: item.title,
    link: item.link,
    pub_date: if let Some(pub_date) = item.pub_date {
      DateTime::parse_from_rfc2822(&pub_date)
        .map(|data| Some(data.to_rfc3339()))
        .unwrap_or(
        DateTime::parse_from_rfc3339(&pub_date)
          .map(|data| Some(data.to_rfc3339()))
          .unwrap_or(None)
        )
    } else {
      None
    },
    content,
    channel_id,
    checksum: "".to_string()
  };

  post.checksum = get_hash(&post);

  return post;
}