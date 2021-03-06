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
pub struct MilestoneBody {
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "due_on")]
  due_on: Option<String>,
  #[serde(rename = "state")]
  state: Option<String>,
  #[serde(rename = "title")]
  title: Option<String>
}

impl MilestoneBody {
  pub fn new() -> MilestoneBody {
    MilestoneBody {
      description: None,
      due_on: None,
      state: None,
      title: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> MilestoneBody {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_due_on(&mut self, due_on: String) {
    self.due_on = Some(due_on);
  }

  pub fn with_due_on(mut self, due_on: String) -> MilestoneBody {
    self.due_on = Some(due_on);
    self
  }

  pub fn due_on(&self) -> Option<&String> {
    self.due_on.as_ref()
  }

  pub fn reset_due_on(&mut self) {
    self.due_on = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> MilestoneBody {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> MilestoneBody {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

}



