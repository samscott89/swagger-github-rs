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
pub struct UserUserId {
  #[serde(rename = "avatar_url")]
  avatar_url: Option<String>,
  #[serde(rename = "bio")]
  bio: Option<String>,
  #[serde(rename = "blog")]
  blog: Option<String>,
  #[serde(rename = "company")]
  company: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  /// Note: The returned email is the user’s publicly visible email address (or null if the user has not specified a public email address in their profile).
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "followers")]
  followers: Option<i32>,
  #[serde(rename = "following")]
  following: Option<i32>,
  #[serde(rename = "gravatar_id")]
  gravatar_id: Option<String>,
  #[serde(rename = "hireable")]
  hireable: Option<bool>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "location")]
  location: Option<String>,
  #[serde(rename = "login")]
  login: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "public_gists")]
  public_gists: Option<i32>,
  #[serde(rename = "public_repos")]
  public_repos: Option<i32>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl UserUserId {
  pub fn new() -> UserUserId {
    UserUserId {
      avatar_url: None,
      bio: None,
      blog: None,
      company: None,
      created_at: None,
      email: None,
      followers: None,
      following: None,
      gravatar_id: None,
      hireable: None,
      html_url: None,
      id: None,
      location: None,
      login: None,
      name: None,
      public_gists: None,
      public_repos: None,
      _type: None,
      url: None
    }
  }

  pub fn set_avatar_url(&mut self, avatar_url: String) {
    self.avatar_url = Some(avatar_url);
  }

  pub fn with_avatar_url(mut self, avatar_url: String) -> UserUserId {
    self.avatar_url = Some(avatar_url);
    self
  }

  pub fn avatar_url(&self) -> Option<&String> {
    self.avatar_url.as_ref()
  }

  pub fn reset_avatar_url(&mut self) {
    self.avatar_url = None;
  }

  pub fn set_bio(&mut self, bio: String) {
    self.bio = Some(bio);
  }

  pub fn with_bio(mut self, bio: String) -> UserUserId {
    self.bio = Some(bio);
    self
  }

  pub fn bio(&self) -> Option<&String> {
    self.bio.as_ref()
  }

  pub fn reset_bio(&mut self) {
    self.bio = None;
  }

  pub fn set_blog(&mut self, blog: String) {
    self.blog = Some(blog);
  }

  pub fn with_blog(mut self, blog: String) -> UserUserId {
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

  pub fn with_company(mut self, company: String) -> UserUserId {
    self.company = Some(company);
    self
  }

  pub fn company(&self) -> Option<&String> {
    self.company.as_ref()
  }

  pub fn reset_company(&mut self) {
    self.company = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> UserUserId {
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

  pub fn with_email(mut self, email: String) -> UserUserId {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_followers(&mut self, followers: i32) {
    self.followers = Some(followers);
  }

  pub fn with_followers(mut self, followers: i32) -> UserUserId {
    self.followers = Some(followers);
    self
  }

  pub fn followers(&self) -> Option<&i32> {
    self.followers.as_ref()
  }

  pub fn reset_followers(&mut self) {
    self.followers = None;
  }

  pub fn set_following(&mut self, following: i32) {
    self.following = Some(following);
  }

  pub fn with_following(mut self, following: i32) -> UserUserId {
    self.following = Some(following);
    self
  }

  pub fn following(&self) -> Option<&i32> {
    self.following.as_ref()
  }

  pub fn reset_following(&mut self) {
    self.following = None;
  }

  pub fn set_gravatar_id(&mut self, gravatar_id: String) {
    self.gravatar_id = Some(gravatar_id);
  }

  pub fn with_gravatar_id(mut self, gravatar_id: String) -> UserUserId {
    self.gravatar_id = Some(gravatar_id);
    self
  }

  pub fn gravatar_id(&self) -> Option<&String> {
    self.gravatar_id.as_ref()
  }

  pub fn reset_gravatar_id(&mut self) {
    self.gravatar_id = None;
  }

  pub fn set_hireable(&mut self, hireable: bool) {
    self.hireable = Some(hireable);
  }

  pub fn with_hireable(mut self, hireable: bool) -> UserUserId {
    self.hireable = Some(hireable);
    self
  }

  pub fn hireable(&self) -> Option<&bool> {
    self.hireable.as_ref()
  }

  pub fn reset_hireable(&mut self) {
    self.hireable = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> UserUserId {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> UserUserId {
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

  pub fn with_location(mut self, location: String) -> UserUserId {
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

  pub fn with_login(mut self, login: String) -> UserUserId {
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

  pub fn with_name(mut self, name: String) -> UserUserId {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_public_gists(&mut self, public_gists: i32) {
    self.public_gists = Some(public_gists);
  }

  pub fn with_public_gists(mut self, public_gists: i32) -> UserUserId {
    self.public_gists = Some(public_gists);
    self
  }

  pub fn public_gists(&self) -> Option<&i32> {
    self.public_gists.as_ref()
  }

  pub fn reset_public_gists(&mut self) {
    self.public_gists = None;
  }

  pub fn set_public_repos(&mut self, public_repos: i32) {
    self.public_repos = Some(public_repos);
  }

  pub fn with_public_repos(mut self, public_repos: i32) -> UserUserId {
    self.public_repos = Some(public_repos);
    self
  }

  pub fn public_repos(&self) -> Option<&i32> {
    self.public_repos.as_ref()
  }

  pub fn reset_public_repos(&mut self) {
    self.public_repos = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> UserUserId {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> UserUserId {
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



