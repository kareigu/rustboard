use crate::db;
use crate::db::types::{DbConn, GetThreads, Thread, User};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;

pub mod types;
mod utils;

#[get("/test")]
pub fn test(db: State<DbConn>) -> Json<Thread> {
  Json(db::get_thread(&db.db, "0x272a".to_string()).thread)
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
pub async fn new_comment(
  mut comment: Form<types::NewComment<'_>>,
  db: State<'_, DbConn>,
) -> std::io::Result<Redirect> {
  let attachment = utils::write_attachment(&mut comment.attachment).await?;

  let new_comment_uid = db::add_comment(&db.db, &comment, attachment);

  Ok(Redirect::to(format!("/t/{}?reply={}", comment.thread, new_comment_uid)))
}

#[post("/thread", data = "<thread>")]
pub async fn new_thread(
  mut thread: Form<types::NewThread<'_>>,
  db: State<'_, DbConn>,
) -> std::io::Result<Redirect> {
  if thread.title.len() > 0 {
    let attachment = utils::write_attachment(&mut thread.attachment).await?;

    let new_thread_uid = db::add_thread(&db.db, thread, attachment);
    Ok(Redirect::to(format!("/t/{}", new_thread_uid)))
  } else {
    Ok(Redirect::to("/post?err=0".to_string()))
  }
}
