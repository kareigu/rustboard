use crate::db;
use crate::db::types::{Attachment, DbConn, GetThreads, Thread, User};
use rocket::form::Form;
use rocket::State;
use rocket_contrib::json::Json;

mod types;
mod utils;

#[get("/test")]
pub fn test(db: State<DbConn>) -> Json<Thread> {
  Json(db::get_thread(&db.db, "0x272a".to_string()))
}

#[get("/users")]
pub fn users(db: State<DbConn>) -> Json<Vec<User>> {
  Json(db::get_users(&db.db))
}

#[get("/threads")]
pub fn threads(db: State<DbConn>) -> Json<GetThreads> {
  Json(db::get_threads(&db.db))
}

#[post("/comment", data = "<comment>")]
pub async fn new_comment(mut comment: Form<types::NewComment<'_>>) -> std::io::Result<String> {
  let attachment = utils::write_attachment(&mut comment.attachment).await?;

  println!("Thread: {}", comment.thread);
  println!("Filename: {}", attachment.filename);
  println!("Content: {}", comment.content);
  Ok("Comment received".to_string())
}
