use serde_derive::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Error {
  #[serde(skip_deserializing)]
  PubSubAuth(goauth::error::GOErr),
  #[serde(skip_deserializing)]
  Json(serde_json::Error),
  #[serde(skip_deserializing)]
  Base64(base64::DecodeError),
  #[serde(skip_deserializing)]
  Unexpected(String),
  #[serde(skip_deserializing)]
  PubSub(String),
  #[serde(skip_deserializing)]
  IOError(std::io::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::PubSubAuth(e) => write!(f, "PubSubAuth({})", e),
      Error::Json(e) => write!(f, "Json({})", e),
      Error::Base64(e) => write!(f, "Base64({})", e),
      Error::Unexpected(message) => write!(f, "Unexpected({})", message),
      Error::PubSub(message) => write!(f, "PubSub({})", message),
      Error::IOError(e) => write!(f, "(IOErr{})", e),
    }
  }
}

impl From<goauth::error::GOErr> for Error {
  fn from(err: goauth::error::GOErr) -> Error {
    Error::PubSubAuth(err)
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Error {
    Error::Json(err)
  }
}

impl std::error::Error for Error {}

// impl From<base64::DecodeError> for Error {
//   fn from(err: base64::DecodeError) -> Error {
//     Error::Base64(err)
//   }
// }
