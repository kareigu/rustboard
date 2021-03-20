pub mod types;

use dgraph::Dgraph;

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

  let resp = db.new_readonly_txn().query(q).expect("Dgraph query");
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

  let resp = db.new_readonly_txn().query(q).expect("Dgraph query");
  let data: types::GetThreads = serde_json::from_slice(&resp.json).expect("parsing");
  data
}
