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
pub struct ParticipationStats {
  #[serde(rename = "all")]
  all: Option<Vec<i32>>,
  #[serde(rename = "owner")]
  owner: Option<Vec<i32>>
}

impl ParticipationStats {
  pub fn new() -> ParticipationStats {
    ParticipationStats {
      all: None,
      owner: None
    }
  }

  pub fn set_all(&mut self, all: Vec<i32>) {
    self.all = Some(all);
  }

  pub fn with_all(mut self, all: Vec<i32>) -> ParticipationStats {
    self.all = Some(all);
    self
  }

  pub fn all(&self) -> Option<&Vec<i32>> {
    self.all.as_ref()
  }

  pub fn reset_all(&mut self) {
    self.all = None;
  }

  pub fn set_owner(&mut self, owner: Vec<i32>) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: Vec<i32>) -> ParticipationStats {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&Vec<i32>> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

}



