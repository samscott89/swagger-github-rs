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
pub struct SearchuserbyemailUser {
  #[serde(rename = "blog")]
  blog: Option<String>,
  #[serde(rename = "company")]
  company: Option<String>,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "followers_count")]
  followers_count: Option<i32>,
  #[serde(rename = "following_count")]
  following_count: Option<i32>,
  #[serde(rename = "gravatar_id")]
  gravatar_id: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "location")]
  location: Option<String>,
  #[serde(rename = "login")]
  login: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "public_gist_count")]
  public_gist_count: Option<i32>,
  #[serde(rename = "public_repo_count")]
  public_repo_count: Option<i32>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl SearchuserbyemailUser {
  pub fn new() -> SearchuserbyemailUser {
    SearchuserbyemailUser {
      blog: None,
      company: None,
      created: None,
      created_at: None,
      email: None,
      followers_count: None,
      following_count: None,
      gravatar_id: None,
      id: None,
      location: None,
      login: None,
      name: None,
      public_gist_count: None,
      public_repo_count: None,
      _type: None
    }
  }

  pub fn set_blog(&mut self, blog: String) {
    self.blog = Some(blog);
  }

  pub fn with_blog(mut self, blog: String) -> SearchuserbyemailUser {
    self.blog = Some(blog);
    self
  }

  pub fn blog(&self) -> Option<&String> {
    self.blog.as_ref()
  }

  pub fn reset_blog(&mut self) {
    self.blog = None;
  }

  pub fn set_company(&mut self, company: String) {
    self.company = Some(company);
  }

  pub fn with_company(mut self, company: String) -> SearchuserbyemailUser {
    self.company = Some(company);
    self
  }

  pub fn company(&self) -> Option<&String> {
    self.company.as_ref()
  }

  pub fn reset_company(&mut self) {
    self.company = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> SearchuserbyemailUser {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> SearchuserbyemailUser {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> SearchuserbyemailUser {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_followers_count(&mut self, followers_count: i32) {
    self.followers_count = Some(followers_count);
  }

  pub fn with_followers_count(mut self, followers_count: i32) -> SearchuserbyemailUser {
    self.followers_count = Some(followers_count);
    self
  }

  pub fn followers_count(&self) -> Option<&i32> {
    self.followers_count.as_ref()
  }

  pub fn reset_followers_count(&mut self) {
    self.followers_count = None;
  }

  pub fn set_following_count(&mut self, following_count: i32) {
    self.following_count = Some(following_count);
  }

  pub fn with_following_count(mut self, following_count: i32) -> SearchuserbyemailUser {
    self.following_count = Some(following_count);
    self
  }

  pub fn following_count(&self) -> Option<&i32> {
    self.following_count.as_ref()
  }

  pub fn reset_following_count(&mut self) {
    self.following_count = None;
  }

  pub fn set_gravatar_id(&mut self, gravatar_id: String) {
    self.gravatar_id = Some(gravatar_id);
  }

  pub fn with_gravatar_id(mut self, gravatar_id: String) -> SearchuserbyemailUser {
    self.gravatar_id = Some(gravatar_id);
    self
  }

  pub fn gravatar_id(&self) -> Option<&String> {
    self.gravatar_id.as_ref()
  }

  pub fn reset_gravatar_id(&mut self) {
    self.gravatar_id = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> SearchuserbyemailUser {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_location(&mut self, location: String) {
    self.location = Some(location);
  }

  pub fn with_location(mut self, location: String) -> SearchuserbyemailUser {
    self.location = Some(location);
    self
  }

  pub fn location(&self) -> Option<&String> {
    self.location.as_ref()
  }

  pub fn reset_location(&mut self) {
    self.location = None;
  }

  pub fn set_login(&mut self, login: String) {
    self.login = Some(login);
  }

  pub fn with_login(mut self, login: String) -> SearchuserbyemailUser {
    self.login = Some(login);
    self
  }

  pub fn login(&self) -> Option<&String> {
    self.login.as_ref()
  }

  pub fn reset_login(&mut self) {
    self.login = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> SearchuserbyemailUser {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_public_gist_count(&mut self, public_gist_count: i32) {
    self.public_gist_count = Some(public_gist_count);
  }

  pub fn with_public_gist_count(mut self, public_gist_count: i32) -> SearchuserbyemailUser {
    self.public_gist_count = Some(public_gist_count);
    self
  }

  pub fn public_gist_count(&self) -> Option<&i32> {
    self.public_gist_count.as_ref()
  }

  pub fn reset_public_gist_count(&mut self) {
    self.public_gist_count = None;
  }

  pub fn set_public_repo_count(&mut self, public_repo_count: i32) {
    self.public_repo_count = Some(public_repo_count);
  }

  pub fn with_public_repo_count(mut self, public_repo_count: i32) -> SearchuserbyemailUser {
    self.public_repo_count = Some(public_repo_count);
    self
  }

  pub fn public_repo_count(&self) -> Option<&i32> {
    self.public_repo_count.as_ref()
  }

  pub fn reset_public_repo_count(&mut self) {
    self.public_repo_count = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> SearchuserbyemailUser {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



