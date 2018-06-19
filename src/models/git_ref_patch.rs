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
pub struct GitRefPatch {
  #[serde(rename = "force")]
  force: Option<bool>,
  #[serde(rename = "sha")]
  sha: Option<String>
}

impl GitRefPatch {
  pub fn new() -> GitRefPatch {
    GitRefPatch {
      force: None,
      sha: None
    }
  }

  pub fn set_force(&mut self, force: bool) {
    self.force = Some(force);
  }

  pub fn with_force(mut self, force: bool) -> GitRefPatch {
    self.force = Some(force);
    self
  }

  pub fn force(&self) -> Option<&bool> {
    self.force.as_ref()
  }

  pub fn reset_force(&mut self) {
    self.force = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> GitRefPatch {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

}



