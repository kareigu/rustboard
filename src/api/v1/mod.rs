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
pub async fn new_comment(
  mut comment: Form<types::NewComment<'_>>,
  db: State<'_, DbConn>,
) -> std::io::Result<Redirect> {
  let attachment_w = utils::write_attachment(&mut comment.attachment).await;

  if let Ok(attachment) = attachment_w {
    if comment.content.len() > 0 || attachment.is_some() {
      Ok(Redirect::to(format!("/t/{}?reply={}", &comment.thread ,db::add_comment(&db.db, &comment, attachment))))
    } else {
      Ok(Redirect::to(format!("/t/{}?reply=reply&err=0", &comment.thread)))
    }
  } else {
    Ok(Redirect::to(format!("/t/{}?reply=reply&err=2", comment.thread)))
  }
}

#[post("/thread", data = "<thread>")]
pub async fn new_thread(
  mut thread: Form<types::NewThread<'_>>,
  db: State<'_, DbConn>,
) -> std::io::Result<Redirect> {
  let attachment_w = utils::write_attachment(&mut thread.attachment).await;

  if let Ok(attachment) = attachment_w {
    if thread.content.len() > 0 {
      Ok(Redirect::to(format!("/{}", db::add_thread(&db.db, thread, attachment))))
    } else {
      Ok(Redirect::to("/post?err=0".to_string()))
    }
  } else {
    Ok(Redirect::to("/post?err=2".to_string()))
  }

}
