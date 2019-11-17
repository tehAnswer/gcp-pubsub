use surf::http::Method;

use crate::presenters::{
  AcknowledgeMessages, CreateSubscription, PullSubscription, ReceiveMessages,
};
use crate::{Client, Error, Message, Topic};

#[derive(Debug)]
pub struct Subscription {
  pub name: String,
  client: crate::Client,
}

impl Subscription {
  pub fn new(client: Client, name: &str) -> Subscription {
    Self {
      name: format!("projects/{}/subscriptions/{}", client.project(), name),
      client,
    }
  }

  pub async fn pull(&mut self) -> Result<Vec<Message>, Error> {
    let url = format!("https://pubsub.googleapis.com/v1/{}:pull", &self.name);
    let payload = PullSubscription::new();

    let response_result = self.client.request(&Method::POST, &url, payload).await;
    let response = self.client.parse::<ReceiveMessages>(response_result).await;

    response.map(|data| data.received_messages)
  }

  pub async fn ack(&mut self, messages: &Vec<Message>) -> Result<(), Error> {
    let url = format!(
      "https://pubsub.googleapis.com/v1/{}:acknowledge",
      &self.name
    );
    let payload = AcknowledgeMessages::new(messages);
    let response_result = self.client.request(&Method::POST, &url, payload).await;
    self
      .client
      .parse::<serde_json::Value>(response_result)
      .await
      .map(|_| ())
  }

  pub async fn create(topic: &Topic, name: &str) -> Result<Subscription, Error> {
    let mut subscription = Self::new(topic.client.clone(), name);
    let url = format!("https://pubsub.googleapis.com/v1/{}", subscription.name);
    let payload = CreateSubscription::from(&topic);
    let client = &mut subscription.client;
    let response_result = client.request(&Method::PUT, &url, payload).await;
    let response = client.parse::<serde_json::Value>(response_result).await;
    response.map(|_| subscription)
  }
}
