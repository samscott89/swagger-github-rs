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
pub struct IssuesComment {
  #[serde(rename = "body")]
  body: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "user")]
  user: Option<::models::BranchCommitAuthor>
}

impl IssuesComment {
  pub fn new() -> IssuesComment {
    IssuesComment {
      body: None,
      created_at: None,
      html_url: None,
      id: None,
      updated_at: None,
      url: None,
      user: None
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> IssuesComment {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> IssuesComment {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> IssuesComment {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> IssuesComment {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> IssuesComment {
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

  pub fn with_url(mut self, url: String) -> IssuesComment {
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

  pub fn with_user(mut self, user: ::models::BranchCommitAuthor) -> IssuesComment {
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



