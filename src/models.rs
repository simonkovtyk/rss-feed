#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Channel {
  pub id: Option<i64>,
  pub name: String,
  pub checksum: String
}

#[derive(serde::Serialize, serde::Deserialize, bincode::Encode, sqlx::FromRow, Debug)]
pub struct Post {
  pub title: Option<String>,
  pub link: Option<String>,
  pub pub_date: Option<String>,
  pub content: Option<String>,
  pub checksum: String,
  pub channel_id: i64
}