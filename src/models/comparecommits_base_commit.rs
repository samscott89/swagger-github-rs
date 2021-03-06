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
pub struct ComparecommitsBaseCommit {
  #[serde(rename = "author")]
  author: Option<::models::ComparecommitsBaseCommitAuthor>,
  #[serde(rename = "commit")]
  commit: Option<::models::ComparecommitsBaseCommitCommit>,
  #[serde(rename = "committer")]
  committer: Option<::models::ComparecommitsBaseCommitAuthor>,
  #[serde(rename = "parents")]
  parents: Option<Vec<::models::BranchCommitCommitTree>>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl ComparecommitsBaseCommit {
  pub fn new() -> ComparecommitsBaseCommit {
    ComparecommitsBaseCommit {
      author: None,
      commit: None,
      committer: None,
      parents: None,
      sha: None,
      url: None
    }
  }

  pub fn set_author(&mut self, author: ::models::ComparecommitsBaseCommitAuthor) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: ::models::ComparecommitsBaseCommitAuthor) -> ComparecommitsBaseCommit {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&::models::ComparecommitsBaseCommitAuthor> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_commit(&mut self, commit: ::models::ComparecommitsBaseCommitCommit) {
    self.commit = Some(commit);
  }

  pub fn with_commit(mut self, commit: ::models::ComparecommitsBaseCommitCommit) -> ComparecommitsBaseCommit {
    self.commit = Some(commit);
    self
  }

  pub fn commit(&self) -> Option<&::models::ComparecommitsBaseCommitCommit> {
    self.commit.as_ref()
  }

  pub fn reset_commit(&mut self) {
    self.commit = None;
  }

  pub fn set_committer(&mut self, committer: ::models::ComparecommitsBaseCommitAuthor) {
    self.committer = Some(committer);
  }

  pub fn with_committer(mut self, committer: ::models::ComparecommitsBaseCommitAuthor) -> ComparecommitsBaseCommit {
    self.committer = Some(committer);
    self
  }

  pub fn committer(&self) -> Option<&::models::ComparecommitsBaseCommitAuthor> {
    self.committer.as_ref()
  }

  pub fn reset_committer(&mut self) {
    self.committer = None;
  }

  pub fn set_parents(&mut self, parents: Vec<::models::BranchCommitCommitTree>) {
    self.parents = Some(parents);
  }

  pub fn with_parents(mut self, parents: Vec<::models::BranchCommitCommitTree>) -> ComparecommitsBaseCommit {
    self.parents = Some(parents);
    self
  }

  pub fn parents(&self) -> Option<&Vec<::models::BranchCommitCommitTree>> {
    self.parents.as_ref()
  }

  pub fn reset_parents(&mut self) {
    self.parents = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> ComparecommitsBaseCommit {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> ComparecommitsBaseCommit {
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



