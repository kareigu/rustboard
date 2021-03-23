use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

use crate::db;
use crate::db::types::DbConn;

#[get("/")]
pub fn index(db: State<DbConn>) -> Template {
  let threads = db::get_threads(&db.db);
  Template::render("index", &threads)
}

#[get("/t/<thread>")]
pub fn thread(db: State<DbConn>, thread: String) -> Template {
  let thread = db::get_thread(&db.db, thread);
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
