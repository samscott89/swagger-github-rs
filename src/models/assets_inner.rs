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
pub struct AssetsInner {
  #[serde(rename = "content_type")]
  content_type: Option<String>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "download_count")]
  download_count: Option<f32>,
  #[serde(rename = "id")]
  id: Option<f32>,
  #[serde(rename = "label")]
  label: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "size")]
  size: Option<f32>,
  #[serde(rename = "state")]
  state: Option<String>,
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "uploader")]
  uploader: Option<::models::AssetUploader>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl AssetsInner {
  pub fn new() -> AssetsInner {
    AssetsInner {
      content_type: None,
      created_at: None,
      download_count: None,
      id: None,
      label: None,
      name: None,
      size: None,
      state: None,
      updated_at: None,
      uploader: None,
      url: None
    }
  }

  pub fn set_content_type(&mut self, content_type: String) {
    self.content_type = Some(content_type);
  }

  pub fn with_content_type(mut self, content_type: String) -> AssetsInner {
    self.content_type = Some(content_type);
    self
  }

  pub fn content_type(&self) -> Option<&String> {
    self.content_type.as_ref()
  }

  pub fn reset_content_type(&mut self) {
    self.content_type = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> AssetsInner {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_download_count(&mut self, download_count: f32) {
    self.download_count = Some(download_count);
  }

  pub fn with_download_count(mut self, download_count: f32) -> AssetsInner {
    self.download_count = Some(download_count);
    self
  }

  pub fn download_count(&self) -> Option<&f32> {
    self.download_count.as_ref()
  }

  pub fn reset_download_count(&mut self) {
    self.download_count = None;
  }

  pub fn set_id(&mut self, id: f32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: f32) -> AssetsInner {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&f32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_label(&mut self, label: String) {
    self.label = Some(label);
  }

  pub fn with_label(mut self, label: String) -> AssetsInner {
    self.label = Some(label);
    self
  }

  pub fn label(&self) -> Option<&String> {
    self.label.as_ref()
  }

  pub fn reset_label(&mut self) {
    self.label = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> AssetsInner {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_size(&mut self, size: f32) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: f32) -> AssetsInner {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&f32> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> AssetsInner {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> AssetsInner {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

  pub fn set_uploader(&mut self, uploader: ::models::AssetUploader) {
    self.uploader = Some(uploader);
  }

  pub fn with_uploader(mut self, uploader: ::models::AssetUploader) -> AssetsInner {
    self.uploader = Some(uploader);
    self
  }

  pub fn uploader(&self) -> Option<&::models::AssetUploader> {
    self.uploader.as_ref()
  }

  pub fn reset_uploader(&mut self) {
    self.uploader = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> AssetsInner {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



