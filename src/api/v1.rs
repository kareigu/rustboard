use crate::db;
use crate::db::types::DbConn;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/test")]
pub fn test(db: State<DbConn>) -> Json<db::types::Thread> {
  Json(db::get_thread(&db.db, "0x272a".to_string()))
}
