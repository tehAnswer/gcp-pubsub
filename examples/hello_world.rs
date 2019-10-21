#[macro_use]
extern crate surf;
extern crate futures;
extern crate gcp_pubsub;
extern crate goauth;
extern crate runtime;
extern crate serde;
extern crate serde_derive;

use futures::future::join_all;
use serde::Serialize;

use gcp_pubsub::Client;
type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Serialize, Default)]
struct X {
  pub a: String,
}

#[runtime::main]
async fn main() -> Result<(), Exception> {
  let file_path = std::env::var("GOOGLE_PUBSUB_CREDENTIALS").unwrap();
  let topic_name = std::env::var("TOPIC").unwrap();

  let credentials = goauth::credentials::Credentials::from_file(&file_path).unwrap();
  let mut client = Client::new(credentials);

  println!("Refreshed token: {}", client.refresh_token().is_ok());
  let topic = client.topic(&topic_name);
  println!("Before sending messages");
  let results = vec![topic.publish(X::default()), topic.publish(X::default())];
  println!("After sending messages");
  let topic_name = nanoid::simple();
  let topic_result = client.create_topic(&topic_name);
  println!("After creating a topic");
  println!("{:?}", join_all(results).await);
  let new_topic = topic_result.await.unwrap();
  println!("{:?}", new_topic);
  let new_subs = new_topic.create_subscription().await.unwrap();
  let msg = new_topic.publish(X::default()).await;
  println!("{:?}", msg);
  println!("{:?}", new_subs);
  let result = new_subs.pull().await;
  println!("{:?}", result);
  Ok(())
}
