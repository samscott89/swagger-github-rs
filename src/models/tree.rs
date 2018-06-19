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
pub struct Tree {
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "tree")]
  tree: Option<Vec<::models::TreeTree>>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      sha: None,
      tree: None,
      url: None
    }
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> Tree {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_tree(&mut self, tree: Vec<::models::TreeTree>) {
    self.tree = Some(tree);
  }

  pub fn with_tree(mut self, tree: Vec<::models::TreeTree>) -> Tree {
    self.tree = Some(tree);
    self
  }

  pub fn tree(&self) -> Option<&Vec<::models::TreeTree>> {
    self.tree.as_ref()
  }

  pub fn reset_tree(&mut self) {
    self.tree = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Tree {
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



