use serde_json::Error;

#[derive(serde::Deserialize)]
pub struct TooManyRequestsResponseBody {
  pub message: String,
  pub retry_after: f32,
  pub global: bool
}

impl TryFrom<String> for TooManyRequestsResponseBody {
  type Error = Error;

  fn try_from(value: String) -> serde_json::Result<Self> {
    serde_json::from_str(&value)
  }
}