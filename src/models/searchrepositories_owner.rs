/* 
 * GitHub
 *
 * Powerful collaboration, code review, and code management for open source and private projects. 
 *
 * OpenAPI spec version: v3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchrepositoriesOwner {
  #[serde(rename = "avatar_url")]
  avatar_url: Option<String>,
  #[serde(rename = "gravatar_id")]
  gravatar_id: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "login")]
  login: Option<String>,
  #[serde(rename = "received_events_url")]
  received_events_url: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl SearchrepositoriesOwner {
  pub fn new() -> SearchrepositoriesOwner {
    SearchrepositoriesOwner {
      avatar_url: None,
      gravatar_id: None,
      id: None,
      login: None,
      received_events_url: None,
      _type: None,
      url: None
    }
  }

  pub fn set_avatar_url(&mut self, avatar_url: String) {
    self.avatar_url = Some(avatar_url);
  }

  pub fn with_avatar_url(mut self, avatar_url: String) -> SearchrepositoriesOwner {
    self.avatar_url = Some(avatar_url);
    self
  }

  pub fn avatar_url(&self) -> Option<&String> {
    self.avatar_url.as_ref()
  }

  pub fn reset_avatar_url(&mut self) {
    self.avatar_url = None;
  }

  pub fn set_gravatar_id(&mut self, gravatar_id: String) {
    self.gravatar_id = Some(gravatar_id);
  }

  pub fn with_gravatar_id(mut self, gravatar_id: String) -> SearchrepositoriesOwner {
    self.gravatar_id = Some(gravatar_id);
    self
  }

  pub fn gravatar_id(&self) -> Option<&String> {
    self.gravatar_id.as_ref()
  }

  pub fn reset_gravatar_id(&mut self) {
    self.gravatar_id = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> SearchrepositoriesOwner {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_login(&mut self, login: String) {
    self.login = Some(login);
  }

  pub fn with_login(mut self, login: String) -> SearchrepositoriesOwner {
    self.login = Some(login);
    self
  }

  pub fn login(&self) -> Option<&String> {
    self.login.as_ref()
  }

  pub fn reset_login(&mut self) {
    self.login = None;
  }

  pub fn set_received_events_url(&mut self, received_events_url: String) {
    self.received_events_url = Some(received_events_url);
  }

  pub fn with_received_events_url(mut self, received_events_url: String) -> SearchrepositoriesOwner {
    self.received_events_url = Some(received_events_url);
    self
  }

  pub fn received_events_url(&self) -> Option<&String> {
    self.received_events_url.as_ref()
  }

  pub fn reset_received_events_url(&mut self) {
    self.received_events_url = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> SearchrepositoriesOwner {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> SearchrepositoriesOwner {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



