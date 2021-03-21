#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::templates::Template;

#[macro_use]
extern crate dgraph;

mod db;
use db::types::DbConn;
mod api;
use api::v1;
mod routes;

fn main() {
  rocket::ignite()
    .manage(DbConn {
      db: make_dgraph!(dgraph::new_dgraph_client("localhost:9080")),
    })
    .attach(Template::fairing())
    .mount(
      "/",
      routes![routes::index, routes::files, routes::ugc, routes::thread],
    )
    .mount("/api/v1/", routes![v1::test])
    .launch();
}
