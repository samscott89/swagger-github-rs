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
pub struct DeploymentStatusesCreate {
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "state")]
  state: Option<String>,
  #[serde(rename = "target_url")]
  target_url: Option<String>
}

impl DeploymentStatusesCreate {
  pub fn new() -> DeploymentStatusesCreate {
    DeploymentStatusesCreate {
      description: None,
      state: None,
      target_url: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> DeploymentStatusesCreate {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> DeploymentStatusesCreate {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_target_url(&mut self, target_url: String) {
    self.target_url = Some(target_url);
  }

  pub fn with_target_url(mut self, target_url: String) -> DeploymentStatusesCreate {
    self.target_url = Some(target_url);
    self
  }

  pub fn target_url(&self) -> Option<&String> {
    self.target_url.as_ref()
  }

  pub fn reset_target_url(&mut self) {
    self.target_url = None;
  }

}



