use anyhow::{Result, Error};
use reqwest::header::{HeaderMap, HeaderValue};

pub async fn get_rss (url: &str) -> Result<bytes::Bytes, Error> {
  let mut headers = HeaderMap::new();
  headers.insert("User-Agent", HeaderValue::from_static(
    "Mozilla/5.0 (X11; Linux x86_64; rv:143.0) Gecko/20100101 Firefox/143.0"
  ));
  headers.insert("Accept", HeaderValue::from_static(
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
  ));
  headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
  headers.insert("Accept-Encoding", HeaderValue::from_static("identity;q=1, *;q=0"));
  headers.insert("Connection", HeaderValue::from_static("keep-alive"));
  headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));

  let client = reqwest::Client::new();
  let bytes = client.get(url)
    .headers(headers)
    .send()
    .await?
    .bytes()
    .await?;

  return Ok(bytes);
}