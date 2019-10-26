use crate::Error;
use crate::Topic;
use goauth::auth::JwtClaims;
use goauth::credentials::Credentials;
use goauth::scopes::Scope;
use smpl_jwt::Jwt;
use surf::http::Method;
use surf::middleware::HttpClient;
use surf::url::Url;
use surf::Request;

#[derive(Clone, Debug)]
pub struct Client {
  credentials: Credentials,
  access_token: Option<String>,
}

impl Client {
  pub fn new(credentials: Credentials) -> Self {
    Client {
      access_token: None,
      credentials,
    }
  }
  pub fn from_file_path(file_path: &str) -> Self {
    let credentials = goauth::credentials::Credentials::from_file(file_path).unwrap();
    Client {
      access_token: None,
      credentials,
    }
  }
  pub fn base_request(&self, method: Method, url: &str) -> Request<impl HttpClient> {
    let parsed_url = Url::parse(url).unwrap();
    Request::new(method, parsed_url).set_header(
      "Authorization",
      format!(
        "Bearer {}",
        self
          .access_token
          .clone()
          .unwrap_or("token_doesnt_exist".into())
      ),
    )
  }

  pub fn topic(&self, name: &str) -> Topic {
    Topic::new(self.clone(), name)
  }

  pub async fn create_topic(&self, name: &str) -> Result<Topic, Error> {
    Topic::create(self.clone(), name).await
  }

  pub fn project(&self) -> String {
    self.credentials.project()
  }

  pub fn refresh_token(&mut self) -> Result<(), Error> {
    match self.get_token() {
      Ok(token) => {
        self.access_token = Some(token.access_token().to_owned());
        Ok(())
      }
      Err(e) => Err(Error::from(e)),
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
