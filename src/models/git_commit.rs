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
pub struct GitCommit {
  #[serde(rename = "author")]
  author: Option<::models::ComparecommitsBaseCommitCommitAuthor>,
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "parents")]
  parents: Option<String>,
  #[serde(rename = "tree")]
  tree: Option<String>
}

impl GitCommit {
  pub fn new() -> GitCommit {
    GitCommit {
      author: None,
      message: None,
      parents: None,
      tree: None
    }
  }

  pub fn set_author(&mut self, author: ::models::ComparecommitsBaseCommitCommitAuthor) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: ::models::ComparecommitsBaseCommitCommitAuthor) -> GitCommit {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&::models::ComparecommitsBaseCommitCommitAuthor> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> GitCommit {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_parents(&mut self, parents: String) {
    self.parents = Some(parents);
  }

  pub fn with_parents(mut self, parents: String) -> GitCommit {
    self.parents = Some(parents);
    self
  }

  pub fn parents(&self) -> Option<&String> {
    self.parents.as_ref()
  }

  pub fn reset_parents(&mut self) {
    self.parents = None;
  }

  pub fn set_tree(&mut self, tree: String) {
    self.tree = Some(tree);
  }

  pub fn with_tree(mut self, tree: String) -> GitCommit {
    self.tree = Some(tree);
    self
  }

  pub fn tree(&self) -> Option<&String> {
    self.tree.as_ref()
  }

  pub fn reset_tree(&mut self) {
    self.tree = None;
  }

}


