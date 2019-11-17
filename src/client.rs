use crate::Error;
use crate::Topic;
use goauth::auth::JwtClaims;
use goauth::credentials::Credentials;
use goauth::scopes::Scope;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use smpl_jwt::Jwt;
use surf::http::Method;
use surf::middleware::HttpClient;
use surf::url::Url;
use surf::Request;
use time;

#[derive(Clone, Debug)]
pub struct Client {
  credentials: Credentials,
  access_token: Option<String>,
  token_expires_at: Option<u64>,
}

impl Client {
  pub fn new(credentials: Credentials) -> Self {
    Client {
      access_token: None,
      token_expires_at: None,
      credentials,
    }
  }
  pub fn from_file_path(file_path: &str) -> Self {
    let credentials = goauth::credentials::Credentials::from_file(file_path).unwrap();
    Client {
      access_token: None,
      token_expires_at: None,
      credentials,
    }
  }
  pub fn base_request(
    &mut self,
    method: &Method,
    url: &str,
  ) -> Result<Request<impl HttpClient>, Error> {
    let parsed_url = Url::parse(url).unwrap();
    let access_token_result = self.fetch_access_token();
    access_token_result.map(|token| {
      Request::new(method.clone(), parsed_url)
        .set_header("Authorization", format!("Bearer {}", token))
    })
  }

  pub fn topic(&self, name: &str) -> Topic {
    Topic::new(self.clone(), name)
  }

  pub async fn request<T: Serialize>(
    &mut self,
    method: &Method,
    url: &str,
    payload: T,
  ) -> Result<surf::Response, Error> {
    let base_request = self.base_request(method, url);
    let request_result =
      base_request.and_then(|request| request.body_json(&payload).map_err(Error::Json));
    if request_result.is_ok() {
      let request = request_result.unwrap();
      request
        .await
        .map_err(|e| Error::Unexpected(format!("Error on request {} {}: {}", method, url, e)))
    } else {
      Err(request_result.err().unwrap())
    }
  }

  pub async fn parse<T: DeserializeOwned>(
    &self,
    response: Result<surf::Response, Error>,
  ) -> Result<T, Error> {
    if response.is_err() {
      Err(response.err().unwrap())
    } else {
      self.parse_result::<T>(&mut response.unwrap()).await
    }
  }

  async fn parse_result<T: DeserializeOwned>(
    &self,
    response: &mut surf::Response,
  ) -> Result<T, Error> {
    let payload = response.body_string().await;
    if response.status().is_success() {
      payload
        .map_err(|err| Error::Unexpected(format!("Error while deserializing: {}", err)))
        .and_then(|payload| serde_json::from_str(&payload).map_err(Error::Json))
    } else {
      payload
        .map_err(|err| Error::Unexpected(format!("Error while deserializing: {}", err)))
        .and_then(|json| Err(Error::PubSub(json)))
    }
  }

  pub async fn create_topic(&self, name: &str) -> Result<Topic, Error> {
    Topic::create(self.clone(), name).await
  }

  pub fn project(&self) -> String {
    self.credentials.project()
  }

  pub fn refresh_token(&mut self) -> Result<String, Error> {
    match self.get_token() {
      Ok(token) => {
        self.token_expires_at =
          Some(time::now_utc().to_timespec().sec as u64 + token.expires_in() as u64);
        self.access_token = Some(token.access_token().to_owned());
        Ok(token.access_token().to_owned())
      }
      Err(e) => Err(Error::from(e)),
    }
  }

  fn fetch_access_token(&mut self) -> Result<String, Error> {
    match self.token_expires_at {
      Some(expires_at) => {
        if time::now_utc().to_timespec().sec as u64 > expires_at {
          self.refresh_token()
        } else {
          Ok(self.access_token.clone().unwrap())
        }
      }
      None => self.refresh_token(),
    }
  }

  fn get_token(&mut self) -> Result<goauth::auth::Token, goauth::error::GOErr> {
    let claims = JwtClaims::new(
      self.credentials.iss(),
      &Scope::PubSub,
      self.credentials.token_uri(),
      None,
      None,
    );
    let jwt = Jwt::new(claims, self.credentials.rsa_key().unwrap(), None);
    goauth::get_token_with_creds(&jwt, &self.credentials)
  }
}
