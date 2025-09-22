use bytes::Bytes;
use anyhow::{Result, Error};

pub async fn get_rss (url: &str) -> Result<bytes::Bytes, Error> {
  let response = reqwest::get(url).await?;

  return Ok(response.bytes().await?);
}