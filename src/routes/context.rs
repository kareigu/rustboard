use serde::Serialize;
use crate::db::types::Thread;

#[derive(Serialize)]
pub struct PostThreadContext {
  pub error_message: Option<String>,
}

#[derive(Serialize)]
pub struct GetThreadContext {
  pub reply: Option<String>,
  pub error_message: Option<String>,
  pub thread: Thread,
}

#[derive(Serialize)]
pub struct GetIndexContext {
  pub error_message: Option<String>,
  pub threads: Vec<Thread>,
}