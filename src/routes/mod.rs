use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

use crate::db;
use crate::db::types::DbConn;

#[get("/")]
pub fn index(db: State<DbConn>) -> Template {
  let mut threads = db::get_threads(&db.db);
  for t in &mut threads.threads {
    t.parse_texts();
  }
  Template::render("index", &threads)
}

#[get("/t/<thread>")]
pub fn thread(db: State<DbConn>, thread: String) -> Template {
  let mut thread = db::get_thread(&db.db, thread);
  thread.parse_texts();

  if let Some(comments) = &mut thread.comments {
    for c in comments {
      c.parse_texts();
    }
  }

  Template::render("thread", &thread)
}

#[get("/static/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/files/<file..>")]
pub async fn ugc(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("files/").join(file)).await.ok()
}

#[get("/post")]
pub fn post_thread() -> Template {
  #[derive(serde::Serialize)]
  struct Temp {
    num: i32,
  }

  let temp = Temp { num: 2 };
  Template::render("create_thread", &temp)
}
