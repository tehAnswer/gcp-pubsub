use crate::Message;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct AcknowledgeMessages {
  #[serde(rename(serialize = "ackIds"))]
  pub message_ids: Vec<String>,
}

impl AcknowledgeMessages {
  pub fn new(messages: &Vec<Message>) -> Self {
    Self {
      message_ids: messages
        .into_iter()
        .map(|message| message.ack_id.clone())
        .collect(),
    }
  }
}
