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
pub struct Events {
  #[serde(rename = "actor")]
  actor: Option<::models::BranchCommitAuthor>,
  #[serde(rename = "created_at")]
  created_at: Option<Value>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "org")]
  org: Option<::models::BranchCommitAuthor>,
  #[serde(rename = "payload")]
  payload: Option<Value>,
  #[serde(rename = "public")]
  public: Option<bool>,
  #[serde(rename = "repo")]
  repo: Option<::models::EventsRepo>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl Events {
  pub fn new() -> Events {
    Events {
      actor: None,
      created_at: None,
      id: None,
      org: None,
      payload: None,
      public: None,
      repo: None,
      _type: None
    }
  }

  pub fn set_actor(&mut self, actor: ::models::BranchCommitAuthor) {
    self.actor = Some(actor);
  }

  pub fn with_actor(mut self, actor: ::models::BranchCommitAuthor) -> Events {
    self.actor = Some(actor);
    self
  }

  pub fn actor(&self) -> Option<&::models::BranchCommitAuthor> {
    self.actor.as_ref()
  }

  pub fn reset_actor(&mut self) {
    self.actor = None;
  }

  pub fn set_created_at(&mut self, created_at: Value) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: Value) -> Events {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&Value> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> Events {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_org(&mut self, org: ::models::BranchCommitAuthor) {
    self.org = Some(org);
  }

  pub fn with_org(mut self, org: ::models::BranchCommitAuthor) -> Events {
    self.org = Some(org);
    self
  }

  pub fn org(&self) -> Option<&::models::BranchCommitAuthor> {
    self.org.as_ref()
  }

  pub fn reset_org(&mut self) {
    self.org = None;
  }

  pub fn set_payload(&mut self, payload: Value) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: Value) -> Events {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&Value> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_public(&mut self, public: bool) {
    self.public = Some(public);
  }

  pub fn with_public(mut self, public: bool) -> Events {
    self.public = Some(public);
    self
  }

  pub fn public(&self) -> Option<&bool> {
    self.public.as_ref()
  }

  pub fn reset_public(&mut self) {
    self.public = None;
  }

  pub fn set_repo(&mut self, repo: ::models::EventsRepo) {
    self.repo = Some(repo);
  }

  pub fn with_repo(mut self, repo: ::models::EventsRepo) -> Events {
    self.repo = Some(repo);
    self
  }

  pub fn repo(&self) -> Option<&::models::EventsRepo> {
    self.repo.as_ref()
  }

  pub fn reset_repo(&mut self) {
    self.repo = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> Events {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



