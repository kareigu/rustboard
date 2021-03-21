use crate::db;
use crate::db::types::{DbConn, Thread, User};
use rocket::State;
use rocket_contrib::json::Json;

#[get("/test")]
pub fn test(db: State<DbConn>) -> Json<Thread> {
  Json(db::get_thread(&db.db, "0x272a".to_string()))
}

#[get("/users")]
pub fn users(db: State<DbConn>) -> Json<Vec<User>> {
  Json(db::get_users(&db.db))
}
