use serde::{Deserialize, Serialize};
use surf::http::Method;

use crate::message::EncodedMessage;
use crate::Client;
use crate::Error;

pub struct Topic {
  client: crate::Client,
  name: String,
}

#[derive(Deserialize, Serialize)]
struct PublishPayload {
  pub messages: Vec<EncodedMessage>,
}

impl PublishPayload {
  pub fn from<T: serde::Serialize>(data: &T) -> Self {
    Self {
      messages: vec![EncodedMessage::new(data)],
    }
  }
}

impl Topic {
  pub fn new(client: Client, name: &str) -> Self {
    Self {
      name: format!("projects/{}/topics/{}", client.project(), name),
      client,
    }
  }

  pub async fn publish<T: Serialize>(&self, data: T) -> Result<(), Error> {
    let url = format!("https://pubsub.googleapis.com/v1/{}:publish", self.name);
    let payload = PublishPayload::from(&data);
    let mut response = self
      .client
      .base_request(Method::POST, &url)
      .body_json(&payload)?
      .await
      .unwrap();
    if response.status().is_success() {
      return Ok(());
    } else {
      response
        .body_string()
        .await
        .map_err(|err| Error::Unexpected(format!("{}", err)))
        .and_then(|json| Err(Error::PubSub(json)))
    }
  }
}
