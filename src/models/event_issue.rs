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
pub struct EventIssue {
  #[serde(rename = "assignee")]
  assignee: Option<::models::BranchCommitAuthor>,
  #[serde(rename = "body")]
  body: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "closed_at")]
  closed_at: Option<String>,
  #[serde(rename = "comments")]
  comments: Option<i32>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "html_url")]
  html_url: Option<String>,
  #[serde(rename = "labels")]
  labels: Option<Vec<::models::EventIssueLabels>>,
  #[serde(rename = "milestone")]
  milestone: Option<::models::EventIssueMilestone>,
  #[serde(rename = "number")]
  number: Option<i32>,
  #[serde(rename = "pull_request")]
  pull_request: Option<::models::EventIssuePullRequest>,
  #[serde(rename = "state")]
  state: Option<::models::ErrorUnknown>,
  #[serde(rename = "title")]
  title: Option<String>,
  /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "user")]
  user: Option<::models::BranchCommitAuthor>
}

impl EventIssue {
  pub fn new() -> EventIssue {
    EventIssue {
      assignee: None,
      body: None,
      closed_at: None,
      comments: None,
      created_at: None,
      html_url: None,
      labels: None,
      milestone: None,
      number: None,
      pull_request: None,
      state: None,
      title: None,
      updated_at: None,
      url: None,
      user: None
    }
  }

  pub fn set_assignee(&mut self, assignee: ::models::BranchCommitAuthor) {
    self.assignee = Some(assignee);
  }

  pub fn with_assignee(mut self, assignee: ::models::BranchCommitAuthor) -> EventIssue {
    self.assignee = Some(assignee);
    self
  }

  pub fn assignee(&self) -> Option<&::models::BranchCommitAuthor> {
    self.assignee.as_ref()
  }

  pub fn reset_assignee(&mut self) {
    self.assignee = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> EventIssue {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_closed_at(&mut self, closed_at: String) {
    self.closed_at = Some(closed_at);
  }

  pub fn with_closed_at(mut self, closed_at: String) -> EventIssue {
    self.closed_at = Some(closed_at);
    self
  }

  pub fn closed_at(&self) -> Option<&String> {
    self.closed_at.as_ref()
  }

  pub fn reset_closed_at(&mut self) {
    self.closed_at = None;
  }

  pub fn set_comments(&mut self, comments: i32) {
    self.comments = Some(comments);
  }

  pub fn with_comments(mut self, comments: i32) -> EventIssue {
    self.comments = Some(comments);
    self
  }

  pub fn comments(&self) -> Option<&i32> {
    self.comments.as_ref()
  }

  pub fn reset_comments(&mut self) {
    self.comments = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> EventIssue {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_html_url(&mut self, html_url: String) {
    self.html_url = Some(html_url);
  }

  pub fn with_html_url(mut self, html_url: String) -> EventIssue {
    self.html_url = Some(html_url);
    self
  }

  pub fn html_url(&self) -> Option<&String> {
    self.html_url.as_ref()
  }

  pub fn reset_html_url(&mut self) {
    self.html_url = None;
  }

  pub fn set_labels(&mut self, labels: Vec<::models::EventIssueLabels>) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: Vec<::models::EventIssueLabels>) -> EventIssue {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&Vec<::models::EventIssueLabels>> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_milestone(&mut self, milestone: ::models::EventIssueMilestone) {
    self.milestone = Some(milestone);
  }

  pub fn with_milestone(mut self, milestone: ::models::EventIssueMilestone) -> EventIssue {
    self.milestone = Some(milestone);
    self
  }

  pub fn milestone(&self) -> Option<&::models::EventIssueMilestone> {
    self.milestone.as_ref()
  }

  pub fn reset_milestone(&mut self) {
    self.milestone = None;
  }

  pub fn set_number(&mut self, number: i32) {
    self.number = Some(number);
  }

  pub fn with_number(mut self, number: i32) -> EventIssue {
    self.number = Some(number);
    self
  }

  pub fn number(&self) -> Option<&i32> {
    self.number.as_ref()
  }

  pub fn reset_number(&mut self) {
    self.number = None;
  }

  pub fn set_pull_request(&mut self, pull_request: ::models::EventIssuePullRequest) {
    self.pull_request = Some(pull_request);
  }

  pub fn with_pull_request(mut self, pull_request: ::models::EventIssuePullRequest) -> EventIssue {
    self.pull_request = Some(pull_request);
    self
  }

  pub fn pull_request(&self) -> Option<&::models::EventIssuePullRequest> {
    self.pull_request.as_ref()
  }

  pub fn reset_pull_request(&mut self) {
    self.pull_request = None;
  }

  pub fn set_state(&mut self, state: ::models::ErrorUnknown) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::ErrorUnknown) -> EventIssue {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::ErrorUnknown> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> EventIssue {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> EventIssue {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> EventIssue {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_user(&mut self, user: ::models::BranchCommitAuthor) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::BranchCommitAuthor) -> EventIssue {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::BranchCommitAuthor> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}


