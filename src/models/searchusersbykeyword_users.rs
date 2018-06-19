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
pub struct SearchusersbykeywordUsers {
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "followers")]
  followers: Option<i32>,
  #[serde(rename = "followers_count")]
  followers_count: Option<i32>,
  #[serde(rename = "fullname")]
  fullname: Option<String>,
  #[serde(rename = "gravatar_id")]
  gravatar_id: Option<String>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "language")]
  language: Option<String>,
  #[serde(rename = "location")]
  location: Option<String>,
  #[serde(rename = "login")]
  login: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "public_repo_count")]
  public_repo_count: Option<i32>,
  #[serde(rename = "repos")]
  repos: Option<i32>,
  #[serde(rename = "score")]
  score: Option<f32>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "username")]
  username: Option<String>
}

impl SearchusersbykeywordUsers {
  pub fn new() -> SearchusersbykeywordUsers {
    SearchusersbykeywordUsers {
      created: None,
      created_at: None,
      followers: None,
      followers_count: None,
      fullname: None,
      gravatar_id: None,
      id: None,
      language: None,
      location: None,
      login: None,
      name: None,
      public_repo_count: None,
      repos: None,
      score: None,
      _type: None,
      username: None
    }
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> SearchusersbykeywordUsers {
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

  pub fn with_created_at(mut self, created_at: String) -> SearchusersbykeywordUsers {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_followers(&mut self, followers: i32) {
    self.followers = Some(followers);
  }

  pub fn with_followers(mut self, followers: i32) -> SearchusersbykeywordUsers {
    self.followers = Some(followers);
    self
  }

  pub fn followers(&self) -> Option<&i32> {
    self.followers.as_ref()
  }

  pub fn reset_followers(&mut self) {
    self.followers = None;
  }

  pub fn set_followers_count(&mut self, followers_count: i32) {
    self.followers_count = Some(followers_count);
  }

  pub fn with_followers_count(mut self, followers_count: i32) -> SearchusersbykeywordUsers {
    self.followers_count = Some(followers_count);
    self
  }

  pub fn followers_count(&self) -> Option<&i32> {
    self.followers_count.as_ref()
  }

  pub fn reset_followers_count(&mut self) {
    self.followers_count = None;
  }

  pub fn set_fullname(&mut self, fullname: String) {
    self.fullname = Some(fullname);
  }

  pub fn with_fullname(mut self, fullname: String) -> SearchusersbykeywordUsers {
    self.fullname = Some(fullname);
    self
  }

  pub fn fullname(&self) -> Option<&String> {
    self.fullname.as_ref()
  }

  pub fn reset_fullname(&mut self) {
    self.fullname = None;
  }

  pub fn set_gravatar_id(&mut self, gravatar_id: String) {
    self.gravatar_id = Some(gravatar_id);
  }

  pub fn with_gravatar_id(mut self, gravatar_id: String) -> SearchusersbykeywordUsers {
    self.gravatar_id = Some(gravatar_id);
    self
  }

  pub fn gravatar_id(&self) -> Option<&String> {
    self.gravatar_id.as_ref()
  }

  pub fn reset_gravatar_id(&mut self) {
    self.gravatar_id = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SearchusersbykeywordUsers {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_language(&mut self, language: String) {
    self.language = Some(language);
  }

  pub fn with_language(mut self, language: String) -> SearchusersbykeywordUsers {
    self.language = Some(language);
    self
  }

  pub fn language(&self) -> Option<&String> {
    self.language.as_ref()
  }

  pub fn reset_language(&mut self) {
    self.language = None;
  }

  pub fn set_location(&mut self, location: String) {
    self.location = Some(location);
  }

  pub fn with_location(mut self, location: String) -> SearchusersbykeywordUsers {
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

  pub fn with_login(mut self, login: String) -> SearchusersbykeywordUsers {
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

  pub fn with_name(mut self, name: String) -> SearchusersbykeywordUsers {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_public_repo_count(&mut self, public_repo_count: i32) {
    self.public_repo_count = Some(public_repo_count);
  }

  pub fn with_public_repo_count(mut self, public_repo_count: i32) -> SearchusersbykeywordUsers {
    self.public_repo_count = Some(public_repo_count);
    self
  }

  pub fn public_repo_count(&self) -> Option<&i32> {
    self.public_repo_count.as_ref()
  }

  pub fn reset_public_repo_count(&mut self) {
    self.public_repo_count = None;
  }

  pub fn set_repos(&mut self, repos: i32) {
    self.repos = Some(repos);
  }

  pub fn with_repos(mut self, repos: i32) -> SearchusersbykeywordUsers {
    self.repos = Some(repos);
    self
  }

  pub fn repos(&self) -> Option<&i32> {
    self.repos.as_ref()
  }

  pub fn reset_repos(&mut self) {
    self.repos = None;
  }

  pub fn set_score(&mut self, score: f32) {
    self.score = Some(score);
  }

  pub fn with_score(mut self, score: f32) -> SearchusersbykeywordUsers {
    self.score = Some(score);
    self
  }

  pub fn score(&self) -> Option<&f32> {
    self.score.as_ref()
  }

  pub fn reset_score(&mut self) {
    self.score = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> SearchusersbykeywordUsers {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_username(&mut self, username: String) {
    self.username = Some(username);
  }

  pub fn with_username(mut self, username: String) -> SearchusersbykeywordUsers {
    self.username = Some(username);
    self
  }

  pub fn username(&self) -> Option<&String> {
    self.username.as_ref()
  }

  pub fn reset_username(&mut self) {
    self.username = None;
  }

}



