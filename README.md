# gcp-pubsub

A crate that acts as a HTTP client to publish and read messages from Google Cloud Platform's PubSub.

## Usage

#### Create a client

Authentication is provided by [rust-goauth](), which expects a path to the file containing your Google Cloud service account JSON key.

```rust
let google_credentials = std::env::var("GOOGLE_PUBSUB_CREDENTIALS").unwrap();
let mut client = gcp_pubsub::Client::new(credentials);
```

#### Create a topic

```rust
let topic = client.create_topic("my-topic").await;
```

#### Publish a message

```rust
#[derive(Serialize, Default)]
struct Foo {
  pub a: String,
}

let result: Result<(), Error> = topic.publish(Foo::default()).await;
```

#### Read messages

```rust
let subscription = topic.create_subscription().await.unwrap();
let messages : Result<Vec<gcp_pubsub::Message>, Error> = subscription.pull().await;
```
