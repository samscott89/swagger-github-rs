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
pub struct HeadsInner {
  #[serde(rename = "commit")]
  commit: Option<::models::BranchCommitCommitTree>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "tarball_url")]
  tarball_url: Option<String>,
  #[serde(rename = "zipball_url")]
  zipball_url: Option<String>
}

impl HeadsInner {
  pub fn new() -> HeadsInner {
    HeadsInner {
      commit: None,
      name: None,
      tarball_url: None,
      zipball_url: None
    }
  }

  pub fn set_commit(&mut self, commit: ::models::BranchCommitCommitTree) {
    self.commit = Some(commit);
  }

  pub fn with_commit(mut self, commit: ::models::BranchCommitCommitTree) -> HeadsInner {
    self.commit = Some(commit);
    self
  }

  pub fn commit(&self) -> Option<&::models::BranchCommitCommitTree> {
    self.commit.as_ref()
  }

  pub fn reset_commit(&mut self) {
    self.commit = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> HeadsInner {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_tarball_url(&mut self, tarball_url: String) {
    self.tarball_url = Some(tarball_url);
  }

  pub fn with_tarball_url(mut self, tarball_url: String) -> HeadsInner {
    self.tarball_url = Some(tarball_url);
    self
  }

  pub fn tarball_url(&self) -> Option<&String> {
    self.tarball_url.as_ref()
  }

  pub fn reset_tarball_url(&mut self) {
    self.tarball_url = None;
  }

  pub fn set_zipball_url(&mut self, zipball_url: String) {
    self.zipball_url = Some(zipball_url);
  }

  pub fn with_zipball_url(mut self, zipball_url: String) -> HeadsInner {
    self.zipball_url = Some(zipball_url);
    self
  }

  pub fn zipball_url(&self) -> Option<&String> {
    self.zipball_url.as_ref()
  }

  pub fn reset_zipball_url(&mut self) {
    self.zipball_url = None;
  }

}


