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
pub struct HookInner {
  #[serde(rename = "active")]
  active: Option<bool>,
  #[serde(rename = "config")]
  config: Option<::models::HookInnerConfig>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "events")]
  events: Option<Vec<::models::ErrorUnknown>>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "name")]
  name: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl HookInner {
  pub fn new() -> HookInner {
    HookInner {
      active: None,
      config: None,
      created_at: None,
      events: None,
      id: None,
      name: None,
      updated_at: None,
      url: None
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> HookInner {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_config(&mut self, config: ::models::HookInnerConfig) {
    self.config = Some(config);
  }

  pub fn with_config(mut self, config: ::models::HookInnerConfig) -> HookInner {
    self.config = Some(config);
    self
  }

  pub fn config(&self) -> Option<&::models::HookInnerConfig> {
    self.config.as_ref()
  }

  pub fn reset_config(&mut self) {
    self.config = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> HookInner {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_events(&mut self, events: Vec<::models::ErrorUnknown>) {
    self.events = Some(events);
  }

  pub fn with_events(mut self, events: Vec<::models::ErrorUnknown>) -> HookInner {
    self.events = Some(events);
    self
  }

  pub fn events(&self) -> Option<&Vec<::models::ErrorUnknown>> {
    self.events.as_ref()
  }

  pub fn reset_events(&mut self) {
    self.events = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> HookInner {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> HookInner {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> HookInner {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> HookInner {
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



