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
pub struct Event {
  #[serde(rename = "actor")]
  actor: Option<::models::BranchCommitAuthor>,
  #[serde(rename = "commit_id")]
  commit_id: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "event")]
  event: Option<String>,
  #[serde(rename = "issue")]
  issue: Option<::models::EventIssue>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl Event {
  pub fn new() -> Event {
    Event {
      actor: None,
      commit_id: None,
      created_at: None,
      event: None,
      issue: None,
      url: None
    }
  }

  pub fn set_actor(&mut self, actor: ::models::BranchCommitAuthor) {
    self.actor = Some(actor);
  }

  pub fn with_actor(mut self, actor: ::models::BranchCommitAuthor) -> Event {
    self.actor = Some(actor);
    self
  }

  pub fn actor(&self) -> Option<&::models::BranchCommitAuthor> {
    self.actor.as_ref()
  }

  pub fn reset_actor(&mut self) {
    self.actor = None;
  }

  pub fn set_commit_id(&mut self, commit_id: String) {
    self.commit_id = Some(commit_id);
  }

  pub fn with_commit_id(mut self, commit_id: String) -> Event {
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

  pub fn with_created_at(mut self, created_at: String) -> Event {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_event(&mut self, event: String) {
    self.event = Some(event);
  }

  pub fn with_event(mut self, event: String) -> Event {
    self.event = Some(event);
    self
  }

  pub fn event(&self) -> Option<&String> {
    self.event.as_ref()
  }

  pub fn reset_event(&mut self) {
    self.event = None;
  }

  pub fn set_issue(&mut self, issue: ::models::EventIssue) {
    self.issue = Some(issue);
  }

  pub fn with_issue(mut self, issue: ::models::EventIssue) -> Event {
    self.issue = Some(issue);
    self
  }

  pub fn issue(&self) -> Option<&::models::EventIssue> {
    self.issue.as_ref()
  }

  pub fn reset_issue(&mut self) {
    self.issue = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Event {
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


