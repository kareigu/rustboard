pub mod types;

use dgraph::Dgraph;
use std::collections::HashMap;

pub fn get_users(db: &Dgraph) -> Vec<types::User> {
  let q = r#"{
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
  }"#;

  let resp = db.new_readonly_txn().query(q).expect("GetUsers Query");
  let data: types::GetUsers = serde_json::from_slice(&resp.json).expect("parsing");
  data.query
}

pub fn get_threads(db: &Dgraph) -> types::GetThreads {
  let q = r#"{
    threads(func: type(thread), orderdesc: post_time) {
      uid
      title
      post_time
      attachment
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
      attachment
    	content
      comment_count : count(comments)
    	comments(orderdesc: post_time) {
        uid
        content
        post_time
      	attachment
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
