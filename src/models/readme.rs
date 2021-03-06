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
pub struct Readme {
  #[serde(rename = "_links")]
  _links: Option<::models::ContentspathLinks>,
  #[serde(rename = "content")]
  content: Option<String>,
  #[serde(rename = "encoding")]
  encoding: Option<String>,
  #[serde(rename = "git_url")]
  git_url: Option<String>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "path")]
  path: Option<String>,
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "size")]
  size: Option<i32>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl Readme {
  pub fn new() -> Readme {
    Readme {
      _links: None,
      content: None,
      encoding: None,
      git_url: None,
      html_url: None,
      name: None,
      path: None,
      sha: None,
      size: None,
      _type: None,
      url: None
    }
  }

  pub fn set__links(&mut self, _links: ::models::ContentspathLinks) {
    self._links = Some(_links);
  }

  pub fn with__links(mut self, _links: ::models::ContentspathLinks) -> Readme {
    self._links = Some(_links);
    self
  }

  pub fn _links(&self) -> Option<&::models::ContentspathLinks> {
    self._links.as_ref()
  }

  pub fn reset__links(&mut self) {
    self._links = None;
  }

  pub fn set_content(&mut self, content: String) {
    self.content = Some(content);
  }

  pub fn with_content(mut self, content: String) -> Readme {
    self.content = Some(content);
    self
  }

  pub fn content(&self) -> Option<&String> {
    self.content.as_ref()
  }

  pub fn reset_content(&mut self) {
    self.content = None;
  }

  pub fn set_encoding(&mut self, encoding: String) {
    self.encoding = Some(encoding);
  }

  pub fn with_encoding(mut self, encoding: String) -> Readme {
    self.encoding = Some(encoding);
    self
  }

  pub fn encoding(&self) -> Option<&String> {
    self.encoding.as_ref()
  }

  pub fn reset_encoding(&mut self) {
    self.encoding = None;
  }

  pub fn set_git_url(&mut self, git_url: String) {
    self.git_url = Some(git_url);
  }

  pub fn with_git_url(mut self, git_url: String) -> Readme {
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

  pub fn with_html_url(mut self, html_url: String) -> Readme {
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

  pub fn with_name(mut self, name: String) -> Readme {
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

  pub fn with_path(mut self, path: String) -> Readme {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> Readme {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set_size(&mut self, size: i32) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i32) -> Readme {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i32> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> Readme {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Readme {
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



