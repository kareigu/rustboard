#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dgraph;

use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

mod db_fetch;

pub struct DbConn {
  db: dgraph::Dgraph,
}

#[get("/")]
fn index(db: State<DbConn>) -> Template {
  let threads = db_fetch::get_threads(&db.db);
  Template::render("index", &threads)
}

#[get("/t/<thread>")]
fn thread(db: State<DbConn>, thread: String) -> Template {
  let thread = db_fetch::get_thread(&db.db, thread);
  Template::render("thread", &thread)
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/files/<file..>")]
fn ugc(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("files/").join(file)).ok()
}

#[get("/json/test")]
fn test(db: State<DbConn>) -> Json<db_fetch::types::Thread> {
  Json(db_fetch::get_thread(&db.db, "0x272a".to_string()))
}

fn main() {
  rocket::ignite()
    .manage(DbConn {
      db: make_dgraph!(dgraph::new_dgraph_client("localhost:9080")),
    })
    .attach(Template::fairing())
    .mount("/", routes![index, files, test, ugc, thread])
    .launch();
}
