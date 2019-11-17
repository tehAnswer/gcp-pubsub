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
  let file_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS_PATH").unwrap();

  let credentials = goauth::credentials::Credentials::from_file(&file_path).unwrap();
  let client = Client::new(credentials);

  let topic_name = format!("r{}", nanoid::simple());
  let topic_result = client.create_topic(&topic_name);
  let new_topic_result = topic_result.await;
  println!("{:?}", new_topic_result);
  let mut new_topic = new_topic_result.unwrap();
  let new_subs_result = new_topic.create_subscription().await;
  let mut new_subs = new_subs_result.unwrap();

  let msg = new_topic.publish(X::default()).await;
  println!("{:?}", msg);
  println!("{:?}", new_subs);
  let messages = new_subs.pull().await.unwrap();
  println!("{:?}", messages);
  let ack = new_subs.ack(&messages).await;
  println!("{:?}", ack);
  Ok(())
}
