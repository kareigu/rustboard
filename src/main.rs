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
mod utils;

#[launch]
fn rocket() -> rocket::Rocket {
  rocket::ignite()
    .manage(DbConn {
      db: make_dgraph!(dgraph::new_dgraph_client("localhost:9080")),
    })
    .attach(Template::fairing())
    .mount(
      "/",
      routes![
        routes::index,
        routes::files,
        routes::ugc,
        routes::thread,
        routes::post_thread
      ],
    )
    .mount(
      "/api/v1/",
      routes![
        v1::users,
        v1::new_comment,
        v1::threads,
        v1::new_thread
      ],
    )
    .register(
      catchers![
        routes::not_found, 
        routes::server_error
      ]
    )
}
