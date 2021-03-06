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
pub struct SearchcodeItems {
  #[serde(rename = "git_url")]
  git_url: Option<String>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "path")]
  path: Option<String>,
  #[serde(rename = "repository")]
  repository: Option<::models::SearchcodeRepository>,
  #[serde(rename = "score")]
  score: Option<f32>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl SearchcodeItems {
  pub fn new() -> SearchcodeItems {
    SearchcodeItems {
      git_url: None,
      html_url: None,
      name: None,
      path: None,
      repository: None,
      score: None,
      sha: None,
      url: None
    }
  }

  pub fn set_git_url(&mut self, git_url: String) {
    self.git_url = Some(git_url);
  }

  pub fn with_git_url(mut self, git_url: String) -> SearchcodeItems {
    self.git_url = Some(git_url);
    self
  }

  pub fn git_url(&self) -> Option<&String> {
    self.git_url.as_ref()
  }

  pub fn reset_git_url(&mut self) {
    self.git_url = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> SearchcodeItems {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> SearchcodeItems {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> SearchcodeItems {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_repository(&mut self, repository: ::models::SearchcodeRepository) {
    self.repository = Some(repository);
  }

  pub fn with_repository(mut self, repository: ::models::SearchcodeRepository) -> SearchcodeItems {
    self.repository = Some(repository);
    self
  }

  pub fn repository(&self) -> Option<&::models::SearchcodeRepository> {
    self.repository.as_ref()
  }

  pub fn reset_repository(&mut self) {
    self.repository = None;
  }

  pub fn set_score(&mut self, score: f32) {
    self.score = Some(score);
  }

  pub fn with_score(mut self, score: f32) -> SearchcodeItems {
    self.score = Some(score);
    self
  }

  pub fn score(&self) -> Option<&f32> {
    self.score.as_ref()
  }

  pub fn reset_score(&mut self) {
    self.score = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> SearchcodeItems {
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

  pub fn with_url(mut self, url: String) -> SearchcodeItems {
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



