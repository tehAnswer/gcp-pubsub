use crate::presenters::EncodedMessage;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
  #[serde(alias = "ackId")]
  pub(crate) ack_id: String,
  pub message: EncodedMessage,
}
