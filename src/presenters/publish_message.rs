use crate::presenters::EncodedMessage;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct PublishMessage {
  pub messages: Vec<EncodedMessage>,
}

impl PublishMessage {
  pub fn from<T: serde::Serialize>(data: &T) -> Self {
    Self {
      messages: vec![EncodedMessage::new(data)],
    }
  }
}
