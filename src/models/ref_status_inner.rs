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
pub struct RefStatusInner {
  #[serde(rename = "commit_url")]
  commit_url: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "repository_url")]
  repository_url: Option<String>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "state")]
  state: Option<String>,
  #[serde(rename = "statuses")]
  statuses: Option<Vec<::models::RefStatusInnerStatuses>>
}

impl RefStatusInner {
  pub fn new() -> RefStatusInner {
    RefStatusInner {
      commit_url: None,
      name: None,
      repository_url: None,
      sha: None,
      state: None,
      statuses: None
    }
  }

  pub fn set_commit_url(&mut self, commit_url: String) {
    self.commit_url = Some(commit_url);
  }

  pub fn with_commit_url(mut self, commit_url: String) -> RefStatusInner {
    self.commit_url = Some(commit_url);
    self
  }

  pub fn commit_url(&self) -> Option<&String> {
    self.commit_url.as_ref()
  }

  pub fn reset_commit_url(&mut self) {
    self.commit_url = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> RefStatusInner {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_repository_url(&mut self, repository_url: String) {
    self.repository_url = Some(repository_url);
  }

  pub fn with_repository_url(mut self, repository_url: String) -> RefStatusInner {
    self.repository_url = Some(repository_url);
    self
  }

  pub fn repository_url(&self) -> Option<&String> {
    self.repository_url.as_ref()
  }

  pub fn reset_repository_url(&mut self) {
    self.repository_url = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> RefStatusInner {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> RefStatusInner {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_statuses(&mut self, statuses: Vec<::models::RefStatusInnerStatuses>) {
    self.statuses = Some(statuses);
  }

  pub fn with_statuses(mut self, statuses: Vec<::models::RefStatusInnerStatuses>) -> RefStatusInner {
    self.statuses = Some(statuses);
    self
  }

  pub fn statuses(&self) -> Option<&Vec<::models::RefStatusInnerStatuses>> {
    self.statuses.as_ref()
  }

  pub fn reset_statuses(&mut self) {
    self.statuses = None;
  }

}



