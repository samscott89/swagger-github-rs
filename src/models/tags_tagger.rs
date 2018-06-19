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
pub struct TagsTagger {
  /// Timestamp of when this object was tagged.
  #[serde(rename = "date")]
  date: Option<String>,
  /// String of the email of the author of the tag.
  #[serde(rename = "email")]
  email: Option<String>,
  /// String of the name of the author of the tag.
  #[serde(rename = "name")]
  name: Option<String>
}

impl TagsTagger {
  pub fn new() -> TagsTagger {
    TagsTagger {
      date: None,
      email: None,
      name: None
    }
  }

  pub fn set_date(&mut self, date: String) {
    self.date = Some(date);
  }

  pub fn with_date(mut self, date: String) -> TagsTagger {
    self.date = Some(date);
    self
  }

  pub fn date(&self) -> Option<&String> {
    self.date.as_ref()
  }

  pub fn reset_date(&mut self) {
    self.date = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> TagsTagger {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> TagsTagger {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



