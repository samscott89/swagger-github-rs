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
pub struct OrganizationAsTeamMember {
  #[serde(rename = "errors")]
  errors: Option<Vec<::models::OrganizationAsTeamMemberErrors>>,
  #[serde(rename = "message")]
  message: Option<String>
}

impl OrganizationAsTeamMember {
  pub fn new() -> OrganizationAsTeamMember {
    OrganizationAsTeamMember {
      errors: None,
      message: None
    }
  }

  pub fn set_errors(&mut self, errors: Vec<::models::OrganizationAsTeamMemberErrors>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<::models::OrganizationAsTeamMemberErrors>) -> OrganizationAsTeamMember {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&Vec<::models::OrganizationAsTeamMemberErrors>> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> OrganizationAsTeamMember {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

}



