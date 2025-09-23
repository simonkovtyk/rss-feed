use anyhow::{Result, Error};
use reqwest::{header::{HeaderMap, HeaderValue}, Response};

pub fn get_spoof_headers () -> HeaderMap {
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

  headers
}

pub async fn get_rss (url: &str, last_modified: Option<String>) -> Result<Response, Error> {
  let mut spoof_headers = get_spoof_headers();

  if let Some(last_modified) = last_modified {
    let last_modified_header = HeaderValue::from_str(last_modified.as_str());

    if let Ok(last_modified_header_value) = last_modified_header {
      spoof_headers.insert("If-Modified-Since", last_modified_header_value);
    }
  }

  let client = reqwest::Client::new();
  let response = client.get(url)
    .headers(spoof_headers)
    .send()
    .await?;

  Ok(response)
}