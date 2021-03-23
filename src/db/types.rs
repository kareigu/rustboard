use serde::{Deserialize, Serialize};

pub struct DbConn {
  pub db: dgraph::Dgraph,
}

#[derive(Serialize, Deserialize)]
pub struct GetUsers {
  pub query: Vec<User>,
}

#[derive(Serialize, Deserialize)]
pub struct GetThreads {
  pub threads: Vec<Thread>,
}

#[derive(Serialize, Deserialize)]
pub struct GetThread {
  pub thread: Vec<Thread>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
  pub uid: Option<String>,
  pub username: Option<String>,
  pub valid_pwd: Option<bool>,
  pub comments: Option<Vec<Comment>>,
  pub threads: Option<Vec<Thread>>,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
  uid: Option<String>,
  content: Option<String>,
  post_time: Option<String>,
  thread: Option<Thread>,
  poster: Option<User>,
  attachment: Option<Attachment>,
}

#[derive(Serialize, Deserialize)]
pub struct Thread {
  pub uid: Option<String>,
  pub comment_count: Option<i32>,
  pub comments: Option<Vec<Comment>>,
  pub content: Option<String>,
  pub post_time: Option<String>,
  pub poster: Option<User>,
  pub title: Option<String>,
  pub attachment: Option<Attachment>,
}

#[derive(Serialize, Deserialize)]
pub struct Attachment {
  pub filename: String,
  pub content_type: String,
}
