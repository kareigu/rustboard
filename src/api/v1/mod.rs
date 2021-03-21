use crate::db;
use crate::db::types::{DbConn, Thread, User};
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;

mod types;

#[get("/test")]
pub fn test(db: State<DbConn>) -> Json<Thread> {
  Json(db::get_thread(&db.db, "0x272a".to_string()))
}

#[get("/users")]
pub fn users(db: State<DbConn>) -> Json<Vec<User>> {
  Json(db::get_users(&db.db))
}

#[post("/comment", data = "<comment>")]
pub fn new_comment(db: State<DbConn>, comment: Form<types::NewComment>) -> &'static str {
  println!("Thread: {}", comment.thread);
  println!("Attachment: {}", comment.attachment);
  println!("Content: {}", comment.content);
  "Comment received"
}
