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
pub struct SearchRepositoriesByKeyword {
  #[serde(rename = "repositories")]
  repositories: Option<Vec<::models::SearchrepositoriesbykeywordRepositories>>
}

impl SearchRepositoriesByKeyword {
  pub fn new() -> SearchRepositoriesByKeyword {
    SearchRepositoriesByKeyword {
      repositories: None
    }
  }

  pub fn set_repositories(&mut self, repositories: Vec<::models::SearchrepositoriesbykeywordRepositories>) {
    self.repositories = Some(repositories);
  }

  pub fn with_repositories(mut self, repositories: Vec<::models::SearchrepositoriesbykeywordRepositories>) -> SearchRepositoriesByKeyword {
    self.repositories = Some(repositories);
    self
  }

  pub fn repositories(&self) -> Option<&Vec<::models::SearchrepositoriesbykeywordRepositories>> {
    self.repositories.as_ref()
  }

  pub fn reset_repositories(&mut self) {
    self.repositories = None;
  }

}



