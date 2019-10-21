use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct CreateTopic {
  pub name: String,
}

impl CreateTopic {
  pub fn from(name: &str) -> Self {
    Self {
      name: name.to_owned(),
    }
  }
}
