use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

use crate::db;
use crate::db::types::DbConn;
mod utils;
mod context;

#[get("/?<err>")]
pub fn index(db: State<DbConn>, err: Option<usize>) -> Template {
  let mut threads = db::get_threads(&db.db);

  if let Some(threads) = &mut threads.threads  {
    for t in threads {
      t.parse_texts();
    }
  }

  let context = context::GetIndexContext {
    error_message: utils::get_create_post_err_msg(err),
    threads: threads.threads,
  };
  Template::render("index", &context)
}

#[get("/t/<thread>?<reply>&<err>")]
pub fn thread(db: State<DbConn>, thread: String, reply: Option<String>, err: Option<usize>) -> Template {
  let mut thread = db::get_thread(&db.db, thread);
  thread.parse_texts();

  if let Some(comments) = &mut thread.comments {
    for c in comments {
      c.parse_texts();
    }
  }

  println!("{:?}", err);

  let get_thread = context::GetThreadContext {
    reply,
    error_message: utils::get_create_post_err_msg(err),
    thread,
  };

  println!("{:?}", get_thread.error_message);

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
  let context = context::PostThreadContext {
    error_message: utils::get_create_post_err_msg(err),
  };

  Template::render("create_thread", &context)
}
