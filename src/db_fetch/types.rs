use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetUsers {
  pub query: Vec<User>,
}

#[derive(Serialize, Deserialize)]
pub struct GetThreads {
  pub threads: Vec<Thread>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
  uid: Option<String>,
  username: Option<String>,
  valid_pwd: Option<bool>,
  comments: Option<Vec<Comment>>,
  threads: Option<Vec<Thread>>,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
  uid: Option<String>,
  content: Option<String>,
  post_time: Option<String>,
  poster: Option<User>,
  attachment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Thread {
  uid: Option<String>,
  comment_count: Option<i32>,
  comments: Option<Vec<Comment>>,
  content: Option<String>,
  post_time: Option<String>,
  poster: Option<User>,
  title: Option<String>,
  attachment: Option<String>,
}
