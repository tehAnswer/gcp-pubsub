use crate::Message;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct ReceiveMessages {
    #[serde(alias = "receivedMessages")]
    pub received_messages: Vec<Message>,
}
