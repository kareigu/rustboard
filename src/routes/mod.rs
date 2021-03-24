use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

use crate::db;
use crate::db::types::DbConn;
mod utils;

#[get("/")]
pub fn index(db: State<DbConn>) -> Template {
  let mut threads = db::get_threads(&db.db);
  for t in &mut threads.threads {
    t.parse_texts();
  }
  Template::render("index", &threads)
}

#[get("/t/<thread>?<reply>")]
pub fn thread(db: State<DbConn>, thread: String, reply: Option<String>) -> Template {
  let mut get_thread = db::get_thread(&db.db, thread);
  get_thread.thread.parse_texts();

  if let Some(comments) = &mut get_thread.thread.comments {
    for c in comments {
      c.parse_texts();
    }
  }

  get_thread.reply = reply;

  Template::render("thread", &get_thread)
}

#[get("/static/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/files/<file..>")]
pub async fn ugc(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("files/").join(file)).await.ok()
}

#[get("/post?<err>")]
pub fn post_thread(err: Option<usize>) -> Template {
  #[derive(serde::Serialize)]
  struct Context {
    error_message: Option<String>,
  }

  let context = Context {
    error_message: utils::get_create_post_err_msg(err),
  };

  Template::render("create_thread", &context)
}
