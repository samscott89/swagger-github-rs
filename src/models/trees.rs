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
pub struct Trees {
  #[serde(rename = "base_tree")]
  base_tree: Option<String>,
  /// SHA1 checksum ID of the object in the tree.
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "tree")]
  tree: Option<Vec<::models::TreesTree>>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl Trees {
  pub fn new() -> Trees {
    Trees {
      base_tree: None,
      sha: None,
      tree: None,
      url: None
    }
  }

  pub fn set_base_tree(&mut self, base_tree: String) {
    self.base_tree = Some(base_tree);
  }

  pub fn with_base_tree(mut self, base_tree: String) -> Trees {
    self.base_tree = Some(base_tree);
    self
  }

  pub fn base_tree(&self) -> Option<&String> {
    self.base_tree.as_ref()
  }

  pub fn reset_base_tree(&mut self) {
    self.base_tree = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> Trees {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_tree(&mut self, tree: Vec<::models::TreesTree>) {
    self.tree = Some(tree);
  }

  pub fn with_tree(mut self, tree: Vec<::models::TreesTree>) -> Trees {
    self.tree = Some(tree);
    self
  }

  pub fn tree(&self) -> Option<&Vec<::models::TreesTree>> {
    self.tree.as_ref()
  }

  pub fn reset_tree(&mut self) {
    self.tree = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Trees {
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



