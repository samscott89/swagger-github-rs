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
pub struct SearchIssuesByKeyword {
  #[serde(rename = "issues")]
  issues: Option<Vec<::models::SearchissuesbykeywordIssues>>
}

impl SearchIssuesByKeyword {
  pub fn new() -> SearchIssuesByKeyword {
    SearchIssuesByKeyword {
      issues: None
    }
  }

  pub fn set_issues(&mut self, issues: Vec<::models::SearchissuesbykeywordIssues>) {
    self.issues = Some(issues);
  }

  pub fn with_issues(mut self, issues: Vec<::models::SearchissuesbykeywordIssues>) -> SearchIssuesByKeyword {
    self.issues = Some(issues);
    self
  }

  pub fn issues(&self) -> Option<&Vec<::models::SearchissuesbykeywordIssues>> {
    self.issues.as_ref()
  }

  pub fn reset_issues(&mut self) {
    self.issues = None;
  }

}


