use crate::Topic;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct CreateSubscription {
  pub topic: String,
}

impl CreateSubscription {
  pub fn from(topic: &Topic) -> Self {
    Self {
      topic: topic.name.to_owned(),
    }
  }
}
