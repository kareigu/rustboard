use crate::db::types::Thread;
use serde::Serialize;

#[derive(Serialize)]
pub struct PostThreadContext {
  pub error_message: Option<String>,
}

#[derive(Serialize)]
pub struct GetThreadContext {
  pub reply: Option<String>,
  pub error_message: Option<String>,
  pub thread: Option<Thread>,
}

#[derive(Serialize)]
pub struct GetIndexContext {
  pub error_message: Option<String>,
  pub threads: Option<Vec<Thread>>,
}

#[derive(Serialize)]
pub struct CatchContext {
  pub error_message: String,
  pub status: i32,
}
