use crate::db;
use crate::db::types::{DbConn, GetThreads, User};
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;

pub mod types;
mod utils;

#[get("/users")]
pub fn users(db: &State<DbConn>) -> Json<Vec<User>> {
  Json(db::get_users(&db.db))
}

#[get("/threads")]
pub fn threads(db: &State<DbConn>) -> Json<GetThreads> {
  Json(db::get_threads(&db.db))
}

#[post("/comment", data = "<comment>")]
pub async fn new_comment(
  mut comment: Form<types::NewComment<'_>>,
  db: &State<DbConn>,
) -> std::io::Result<Redirect> {
  //* Only handle the request if the supplied UID is of valid format
  //* Eliminates possibility of database code injection this way
  if !crate::utils::check_uid_validity(&comment.thread) {
    //* Redirect user back to the index if an invalid thread UID was supplied
    //* This should never happen under normal circumstances, so it's most probable the user did something stupid
    //* ie. tried to manually change the UID to an invalid value
    return Ok(Redirect::to("/?err=1".to_string()));
  }
  let attachment = match utils::write_attachment(&mut comment.attachment).await {
    Ok(a) => a,
    Err(e) => {
      println!("{}", e);
      return Ok(Redirect::to(format!(
        "/t/{}?reply=reply&err=2",
        comment.thread
      )));
    }
  };

  //* Comments should be allowed to have a message, an attachment or both
  if !comment.content.is_empty() || attachment.is_some() {
    Ok(Redirect::to(format!(
      "/t/{}?reply={}",
      &comment.thread,
      db::add_comment(&db.db, &comment, attachment)
    )))
  } else {
    Ok(Redirect::to(format!(
      "/t/{}?reply=reply&err=0",
      &comment.thread
    )))
  }
}

#[post("/thread", data = "<thread>")]
pub async fn new_thread(
  mut thread: Form<types::NewThread<'_>>,
  db: &State<DbConn>,
) -> std::io::Result<Redirect> {
  if thread.content.is_empty() {
    return Ok(Redirect::to("/post?err=0".to_string()));
  }
  let attachment = match utils::write_attachment(&mut thread.attachment).await {
    Ok(a) => a,
    Err(e) => {
      println!("{}", e);
      return Ok(Redirect::to("/post?err=2".to_string()));
    }
  };

  //* All threads should have some form of message
  Ok(Redirect::to(format!(
    "/{}",
    db::add_thread(&db.db, thread, attachment)
  )))
}
