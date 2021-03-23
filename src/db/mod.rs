pub mod types;
mod utils;

use crate::api::v1::types::{NewComment, NewThread};
use dgraph::Dgraph;
use rocket::form::Form;
use std::collections::HashMap;

pub fn get_users(db: &Dgraph) -> Vec<types::User> {
  /*let q = r#"{
    query(func: type(user)) {
      uid
      username
      valid_pwd : checkpwd(password, "asd123")
      threads {
        uid
        title
        content
        post_time
        poster {
          username
        }
      }
      comments {
        uid
        content
        post_time
        poster {
          username
        }
      }
    }
  }"#;*/

  let q = r#"{
    query(func: type(User)) {
      uid
      username
      valid_pwd : checkpwd(password, "asd123")
    }
  }"#;

  let resp = db.new_readonly_txn().query(q).expect("GetUsers Query");
  let data: types::GetUsers = serde_json::from_slice(&resp.json).expect("parsing");
  data.query
}

pub fn get_threads(db: &Dgraph) -> types::GetThreads {
  let q = r#"{
    threads(func: type(Thread), orderdesc: post_time) {
      uid
      title
      post_time
      attachment {
        filename
        content_type
      }
      comment_count : count(comments)
    }
  }"#;

  let resp = db.new_readonly_txn().query(q).expect("GetThreads Query");
  let data: types::GetThreads = serde_json::from_slice(&resp.json).expect("parsing");
  data
}

pub fn get_thread(db: &Dgraph, uid: String) -> types::Thread {
  let q = r#"query thread($a: string) {
    thread(func: uid($a)) {
      uid
      title
      post_time
      attachment {
        filename
        content_type
      }
    	content
      comment_count : count(comments)
    	comments(orderasc: post_time) {
        uid
        content
        post_time
      	attachment {
          filename
          content_type
        }
      }
    }
  }"#;
  let mut vars = HashMap::new();
  vars.insert("$a".to_string(), uid);

  let resp = db
    .new_readonly_txn()
    .query_with_vars(q, vars)
    .expect("GetThread Query");
  let data: types::GetThread = serde_json::from_slice(&resp.json).expect("parsing");
  let thread = data
    .thread
    .into_iter()
    .next()
    .expect("Couldn't iterate over GetThread Vec");
  thread
}

pub fn add_comment(db: &Dgraph, comment: Form<NewComment>, attachment: Option<types::Attachment>) {
  let mut txn = db.new_txn();

  let mut q = format!(
    r#"
  _:new_comment <content> "{content}" .
  _:new_comment <poster> <{poster_uid}> .
  _:new_comment <post_time> "{post_time}" .
  _:new_comment <thread> <{thread}> .
  _:new_comment <dgraph.type> "Comment" .
  <{thread}> <comments> _:new_comment .
  "#,
    content = comment.content.escape_default(),
    thread = comment.thread,
    poster_uid = 0x2731,
    post_time = utils::get_curr_timestamp()
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
}

pub fn add_thread(db: &Dgraph, thread: Form<NewThread>, attachment: Option<types::Attachment>) {
  let mut txn = db.new_txn();

  let mut q = format!(
    r#"
  _:new_thread <content> "{content}" .
  _:new_thread <poster> <{poster_uid}> .
  _:new_thread <post_time> "{post_time}" .
  _:new_thread <title> "{title}" .
  _:new_thread <dgraph.type> "Thread" .
  "#,
    content = thread.content.escape_default(),
    title = thread.title.escape_default(),
    poster_uid = 0x2731,
    post_time = utils::get_curr_timestamp()
  );

  match attachment {
    Some(a) => q.push_str(
      format!(
        r#"
      _:new_attachment <filename> "{filename}" .
      _:new_attachment <content_type> "{content_type}" .
      _:new_thread <attachment> _:new_attachment .
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
}
