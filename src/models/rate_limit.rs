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
pub struct RateLimit {
  #[serde(rename = "rate")]
  rate: Option<::models::RateLimitRate>
}

impl RateLimit {
  pub fn new() -> RateLimit {
    RateLimit {
      rate: None
    }
  }

  pub fn set_rate(&mut self, rate: ::models::RateLimitRate) {
    self.rate = Some(rate);
  }

  pub fn with_rate(mut self, rate: ::models::RateLimitRate) -> RateLimit {
    self.rate = Some(rate);
    self
  }

  pub fn rate(&self) -> Option<&::models::RateLimitRate> {
    self.rate.as_ref()
  }

  pub fn reset_rate(&mut self) {
    self.rate = None;
  }

}



