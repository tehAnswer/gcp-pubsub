use serde::Serialize;
use surf::http::Method;

use crate::presenters::{CreateTopic, PublishMessage};
use crate::Client;
use crate::Error;

#[derive(Debug)]
pub struct Topic {
  client: crate::Client,
  name: String,
}

impl Topic {
  pub fn new(client: Client, name: &str) -> Self {
    Self {
      name: format!("projects/{}/topics/{}", client.project(), name),
      client,
    }
  }

  pub async fn create(client: Client, name: &str) -> Result<Topic, Error> {
    let topic = Self::new(client, name);
    let url = format!("https://pubsub.googleapis.com/v1/{}", topic.name);
    let payload = CreateTopic::from(&topic.name);
    let mut response = topic
      .client
      .base_request(Method::PUT, &url)
      .body_json(&payload)?
      .await
      .unwrap();
    if response.status().is_success() {
      return Ok(topic);
    } else {
      response
        .body_string()
        .await
        .map_err(|err| Error::Unexpected(format!("{}", err)))
        .and_then(|json| Err(Error::PubSub(json)))
    }
  }

  pub async fn publish<T: Serialize>(&self, data: T) -> Result<(), Error> {
    let url = format!("https://pubsub.googleapis.com/v1/{}:publish", self.name);
    let payload = PublishMessage::from(&data);
    let mut response = self
      .client
      .base_request(Method::POST, &url)
      .body_json(&payload)?
      .await
      .unwrap();
    if response.status().is_success() {
      Ok(())
    } else {
      response
        .body_string()
        .await
        .map_err(|err| Error::Unexpected(format!("{}", err)))
        .and_then(|json| Err(Error::PubSub(json)))
    }
  }
}
