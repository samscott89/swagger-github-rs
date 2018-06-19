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
pub struct CommitBody {
  #[serde(rename = "body")]
  body: String,
  /// Deprecated - Use position parameter instead.
  #[serde(rename = "line")]
  line: Option<String>,
  /// Line number in the file to comment on. Defaults to null.
  #[serde(rename = "number")]
  number: Option<String>,
  /// Relative path of the file to comment on.
  #[serde(rename = "path")]
  path: Option<String>,
  /// Line index in the diff to comment on.
  #[serde(rename = "position")]
  position: Option<i32>,
  /// SHA of the commit to comment on.
  #[serde(rename = "sha")]
  sha: String
}

impl CommitBody {
  pub fn new(body: String, sha: String) -> CommitBody {
    CommitBody {
      body: body,
      line: None,
      number: None,
      path: None,
      position: None,
      sha: sha
    }
  }

  pub fn set_body(&mut self, body: String) {
    self.body = body;
  }

  pub fn with_body(mut self, body: String) -> CommitBody {
    self.body = body;
    self
  }

  pub fn body(&self) -> &String {
    &self.body
  }


  pub fn set_line(&mut self, line: String) {
    self.line = Some(line);
  }

  pub fn with_line(mut self, line: String) -> CommitBody {
    self.line = Some(line);
    self
  }

  pub fn line(&self) -> Option<&String> {
    self.line.as_ref()
  }

  pub fn reset_line(&mut self) {
    self.line = None;
  }

  pub fn set_number(&mut self, number: String) {
    self.number = Some(number);
  }

  pub fn with_number(mut self, number: String) -> CommitBody {
    self.number = Some(number);
    self
  }

  pub fn number(&self) -> Option<&String> {
    self.number.as_ref()
  }

  pub fn reset_number(&mut self) {
    self.number = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> CommitBody {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_position(&mut self, position: i32) {
    self.position = Some(position);
  }

  pub fn with_position(mut self, position: i32) -> CommitBody {
    self.position = Some(position);
    self
  }

  pub fn position(&self) -> Option<&i32> {
    self.position.as_ref()
  }

  pub fn reset_position(&mut self) {
    self.position = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = sha;
  }

  pub fn with_sha(mut self, sha: String) -> CommitBody {
    self.sha = sha;
    self
  }

  pub fn sha(&self) -> &String {
    &self.sha
  }


}


