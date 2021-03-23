use crate::db;
use crate::db::types::{Attachment, DbConn, GetThreads, MutComment, Thread, User};
use rocket::form::Form;
use rocket::State;
use rocket_contrib::json::Json;

mod types;
mod utils;

#[get("/test")]
pub fn test(db: State<DbConn>) -> Json<Thread> {
  Json(db::get_thread(&db.db, "0x272a".to_string()))
}

#[get("/users")]
pub fn users(db: State<DbConn>) -> Json<Vec<User>> {
  Json(db::get_users(&db.db))
}

#[get("/threads")]
pub fn threads(db: State<DbConn>) -> Json<GetThreads> {
  Json(db::get_threads(&db.db))
}

#[post("/comment", data = "<comment>")]
pub async fn new_comment(
  mut comment: Form<types::NewComment<'_>>,
  db: State<'_, DbConn>,
) -> std::io::Result<String> {
  let attachment = utils::write_attachment(&mut comment.attachment).await?;

  let mut txn = db.db.new_txn();

  let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

  let mut q = format!(
    r#"
  _:new_comment <content> "{content}" .
  _:new_comment <poster> <{poster_uid}> .
  _:new_comment <post_time> "{post_time}" .
  _:new_comment <thread> <{thread}> .
  _:new_comment <dgraph.type> "comment" .
  <{thread}> <comments> _:new_comment .
  "#,
    content = comment.content,
    thread = comment.thread,
    poster_uid = 0x2731,
    post_time = now
  );

  match attachment {
    Some(a) => q.push_str(
      format!(
        r#"
      _:new_attachment <filename> "{filename}" .
      _:new_attachment <content_type> "{content_type}" .
      _:new_comment <attachment> _:new_attachment .
    "#,
        filename = a.filename,
        content_type = a.content_type
      )
      .as_str(),
    ),
    None => println!("No attachment"),
  }

  let mut m = dgraph::Mutation::new();
  m.set_set_nquads(q.into());

  let assigned = txn.mutate(m).expect("failed to create data");
  for (key, val) in assigned.uids.iter() {
    println!("\t{} => {}", key, val);
  }
  txn.commit().expect("Transaction committed");

  Ok("Comment received".to_string())
}
