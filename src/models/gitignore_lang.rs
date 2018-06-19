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
pub struct GitignoreLang {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "source")]
  source: Option<String>
}

impl GitignoreLang {
  pub fn new() -> GitignoreLang {
    GitignoreLang {
      name: None,
      source: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GitignoreLang {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_source(&mut self, source: String) {
    self.source = Some(source);
  }

  pub fn with_source(mut self, source: String) -> GitignoreLang {
    self.source = Some(source);
    self
  }

  pub fn source(&self) -> Option<&String> {
    self.source.as_ref()
  }

  pub fn reset_source(&mut self) {
    self.source = None;
  }

}



