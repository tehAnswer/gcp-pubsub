use serde::Serialize;
use surf::http::Method;

use crate::presenters::{CreateSubscription, PullSubscription, ReceiveMessages};
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

  pub async fn pull(&self) -> Result<Vec<Message>, Error> {
    let url = format!("https://pubsub.googleapis.com/v1/{}:pull", &self.name);
    let payload = PullSubscription::new();
    let mut response = self
      .client
      .base_request(Method::POST, &url)
      .body_json(&payload)?
      .await
      .unwrap();
    if response.status().is_success() {
      let body_json = response.body_string().await;

      body_json
        .map_err(|_| Error::Unexpected("Deserializing".into()))
        .and_then(|payload| {
          let result: Result<ReceiveMessages, serde_json::Error> = serde_json::from_str(&payload);
          result
            .and_then(|messages| Ok(messages.received_messages))
            .map_err(Error::Json)
        })
    } else {
      response
        .body_string()
        .await
        .map_err(|err| Error::Unexpected(format!("{}", err)))
        .and_then(|json| Err(Error::PubSub(json)))
    }
  }

  pub async fn create(topic: &Topic) -> Result<Subscription, Error> {
    let new_subscription_name = format!("s{}", &nanoid::generate(10));
    let subscription = Self::new(topic.client.clone(), &new_subscription_name);
    let url = format!("https://pubsub.googleapis.com/v1/{}", subscription.name);
    let payload = CreateSubscription::from(&topic);
    let mut response = subscription
      .client
      .base_request(Method::PUT, &url)
      .body_json(&payload)?
      .await
      .unwrap();
    if response.status().is_success() {
      return Ok(subscription);
    } else {
      response
        .body_string()
        .await
        .map_err(|err| Error::Unexpected(format!("{}", err)))
        .and_then(|json| Err(Error::PubSub(json)))
    }
  }
}
