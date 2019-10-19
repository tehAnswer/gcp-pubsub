use crate::presenters::EncodedMessage;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Message {
  #[serde(alias = "ackId")]
  pub(crate) ack_id: String,
  pub(crate) message: EncodedMessage,
}
