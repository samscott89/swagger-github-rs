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
pub struct IssuesCommentsInnerLinks {
  #[serde(rename = "html")]
  html: Option<::models::IssuesCommentsInnerLinksHtml>,
  #[serde(rename = "pull_request")]
  pull_request: Option<::models::IssuesCommentsInnerLinksHtml>,
  #[serde(rename = "self")]
  _self: Option<::models::IssuesCommentsInnerLinksHtml>
}

impl IssuesCommentsInnerLinks {
  pub fn new() -> IssuesCommentsInnerLinks {
    IssuesCommentsInnerLinks {
      html: None,
      pull_request: None,
      _self: None
    }
  }

  pub fn set_html(&mut self, html: ::models::IssuesCommentsInnerLinksHtml) {
    self.html = Some(html);
  }

  pub fn with_html(mut self, html: ::models::IssuesCommentsInnerLinksHtml) -> IssuesCommentsInnerLinks {
    self.html = Some(html);
    self
  }

  pub fn html(&self) -> Option<&::models::IssuesCommentsInnerLinksHtml> {
    self.html.as_ref()
  }

  pub fn reset_html(&mut self) {
    self.html = None;
  }

  pub fn set_pull_request(&mut self, pull_request: ::models::IssuesCommentsInnerLinksHtml) {
    self.pull_request = Some(pull_request);
  }

  pub fn with_pull_request(mut self, pull_request: ::models::IssuesCommentsInnerLinksHtml) -> IssuesCommentsInnerLinks {
    self.pull_request = Some(pull_request);
    self
  }

  pub fn pull_request(&self) -> Option<&::models::IssuesCommentsInnerLinksHtml> {
    self.pull_request.as_ref()
  }

  pub fn reset_pull_request(&mut self) {
    self.pull_request = None;
  }

  pub fn set__self(&mut self, _self: ::models::IssuesCommentsInnerLinksHtml) {
    self._self = Some(_self);
  }

  pub fn with__self(mut self, _self: ::models::IssuesCommentsInnerLinksHtml) -> IssuesCommentsInnerLinks {
    self._self = Some(_self);
    self
  }

  pub fn _self(&self) -> Option<&::models::IssuesCommentsInnerLinksHtml> {
    self._self.as_ref()
  }

  pub fn reset__self(&mut self) {
    self._self = None;
  }

}



