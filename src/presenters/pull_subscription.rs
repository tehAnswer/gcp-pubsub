use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct PullSubscription {
  pub return_immediately: bool,
  pub max_messages: i128,
}

impl PullSubscription {
  pub fn new() -> Self {
    Self {
      return_immediately: true,
      max_messages: 100,
    }
  }
}
