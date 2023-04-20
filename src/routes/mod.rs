use rocket::fs::NamedFile;
use rocket::State;
use rocket_dyn_templates::Template;
use std::path::{Path, PathBuf};

use crate::db;
use crate::db::types::DbConn;
mod context;
mod utils;

#[get("/?<err>")]
pub fn index(db: &State<DbConn>, err: Option<usize>) -> Template {
  let mut threads = db::get_threads(&db.db);

  if let Some(threads) = &mut threads.threads {
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
pub fn thread(
  db: &State<DbConn>,
  thread: String,
  reply: Option<String>,
  err: Option<usize>,
) -> Template {
  let mut thread = db::get_thread(&db.db, thread);

  if let Some(t) = &mut thread {
    t.parse_texts();

    if let Some(comments) = &mut t.comments {
      for c in comments {
        c.parse_texts();
      }
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

#[catch(404)]
pub fn not_found() -> Template {
  let context = context::CatchContext {
    error_message: "That page doesn't exist".to_string(),
    status: 404,
  };

  Template::render("catcher", &context)
}

#[catch(500)]
pub fn server_error() -> Template {
  let context = context::CatchContext {
    error_message: "Server error".to_string(),
    status: 500,
  };

  Template::render("catcher", &context)
}
