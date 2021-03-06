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
pub struct Markdown {
  #[serde(rename = "context")]
  context: Option<String>,
  #[serde(rename = "mode")]
  mode: Option<String>,
  #[serde(rename = "text")]
  text: Option<String>
}

impl Markdown {
  pub fn new() -> Markdown {
    Markdown {
      context: None,
      mode: None,
      text: None
    }
  }

  pub fn set_context(&mut self, context: String) {
    self.context = Some(context);
  }

  pub fn with_context(mut self, context: String) -> Markdown {
    self.context = Some(context);
    self
  }

  pub fn context(&self) -> Option<&String> {
    self.context.as_ref()
  }

  pub fn reset_context(&mut self) {
    self.context = None;
  }

  pub fn set_mode(&mut self, mode: String) {
    self.mode = Some(mode);
  }

  pub fn with_mode(mut self, mode: String) -> Markdown {
    self.mode = Some(mode);
    self
  }

  pub fn mode(&self) -> Option<&String> {
    self.mode.as_ref()
  }

  pub fn reset_mode(&mut self) {
    self.mode = None;
  }

  pub fn set_text(&mut self, text: String) {
    self.text = Some(text);
  }

  pub fn with_text(mut self, text: String) -> Markdown {
    self.text = Some(text);
    self
  }

  pub fn text(&self) -> Option<&String> {
    self.text.as_ref()
  }

  pub fn reset_text(&mut self) {
    self.text = None;
  }

}



