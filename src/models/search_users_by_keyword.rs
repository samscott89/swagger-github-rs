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
pub struct SearchUsersByKeyword {
  #[serde(rename = "users")]
  users: Option<Vec<::models::SearchusersbykeywordUsers>>
}

impl SearchUsersByKeyword {
  pub fn new() -> SearchUsersByKeyword {
    SearchUsersByKeyword {
      users: None
    }
  }

  pub fn set_users(&mut self, users: Vec<::models::SearchusersbykeywordUsers>) {
    self.users = Some(users);
  }

  pub fn with_users(mut self, users: Vec<::models::SearchusersbykeywordUsers>) -> SearchUsersByKeyword {
    self.users = Some(users);
    self
  }

  pub fn users(&self) -> Option<&Vec<::models::SearchusersbykeywordUsers>> {
    self.users.as_ref()
  }

  pub fn reset_users(&mut self) {
    self.users = None;
  }

}


