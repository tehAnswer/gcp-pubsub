use serde::Serialize;
use surf::http::Method;

use crate::presenters::{CreateTopic, PublishMessage};
use crate::Client;
use crate::Error;
use crate::Subscription;

#[derive(Debug)]
pub struct Topic {
  pub name: String,
  pub(crate) client: crate::Client,
}

impl Topic {
  pub fn new(client: Client, name: &str) -> Self {
    Self {
      name: format!("projects/{}/topics/{}", client.project(), name),
      client,
    }
  }

  pub async fn create(client: Client, name: &str) -> Result<Topic, Error> {
    let mut topic = Self::new(client, name);
    let url = format!("https://pubsub.googleapis.com/v1/{}", topic.name);
    let payload = CreateTopic::from(&topic.name);
    let topic_client = &mut topic.client;
    let response_result = topic_client.request(&Method::PUT, &url, payload).await;
    let response = topic_client.parse::<serde_json::Value>(response_result).await;
    response.map(|_| topic)
  }

  pub async fn create_subscription(&self) -> Result<Subscription, Error> {
    let new_subscription_name = format!("s{}", &nanoid::generate(10));
    Subscription::create(self.clone(), &new_subscription_name).await
  }

  pub async fn create_subscription_with_name(&self, name: &str) -> Result<Subscription, Error> {
    Subscription::create(self.clone(), name).await
  }

  pub async fn publish<T: Serialize>(&mut self, data: T) -> Result<(), Error> {
    let url = format!("https://pubsub.googleapis.com/v1/{}:publish", self.name);
    let payload = PublishMessage::from(&data);

    let response_result = self.client.request(&Method::POST, &url, payload).await;
    self
      .client
      .parse::<serde_json::Value>(response_result)
      .await
      .map(|_| ())
  }
}
