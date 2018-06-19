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
pub struct IssuesCommentsInner {
  #[serde(rename = "_links")]
  _links: Option<::models::IssuesCommentsInnerLinks>,
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "commit_id")]
  commit_id: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "path")]
  path: Option<String>,
  #[serde(rename = "position")]
  position: Option<i32>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "user")]
  user: Option<::models::BranchCommitAuthor>
}

impl IssuesCommentsInner {
  pub fn new() -> IssuesCommentsInner {
    IssuesCommentsInner {
      _links: None,
      body: None,
      commit_id: None,
      created_at: None,
      id: None,
      path: None,
      position: None,
      updated_at: None,
      url: None,
      user: None
    }
  }

  pub fn set__links(&mut self, _links: ::models::IssuesCommentsInnerLinks) {
    self._links = Some(_links);
  }

  pub fn with__links(mut self, _links: ::models::IssuesCommentsInnerLinks) -> IssuesCommentsInner {
    self._links = Some(_links);
    self
  }

  pub fn _links(&self) -> Option<&::models::IssuesCommentsInnerLinks> {
    self._links.as_ref()
  }

  pub fn reset__links(&mut self) {
    self._links = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> IssuesCommentsInner {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_commit_id(&mut self, commit_id: String) {
    self.commit_id = Some(commit_id);
  }

  pub fn with_commit_id(mut self, commit_id: String) -> IssuesCommentsInner {
    self.commit_id = Some(commit_id);
    self
  }

  pub fn commit_id(&self) -> Option<&String> {
    self.commit_id.as_ref()
  }

  pub fn reset_commit_id(&mut self) {
    self.commit_id = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> IssuesCommentsInner {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> IssuesCommentsInner {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> IssuesCommentsInner {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_position(&mut self, position: i32) {
    self.position = Some(position);
  }

  pub fn with_position(mut self, position: i32) -> IssuesCommentsInner {
    self.position = Some(position);
    self
  }

  pub fn position(&self) -> Option<&i32> {
    self.position.as_ref()
  }

  pub fn reset_position(&mut self) {
    self.position = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> IssuesCommentsInner {
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

  pub fn with_url(mut self, url: String) -> IssuesCommentsInner {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_user(&mut self, user: ::models::BranchCommitAuthor) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::BranchCommitAuthor) -> IssuesCommentsInner {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::BranchCommitAuthor> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



