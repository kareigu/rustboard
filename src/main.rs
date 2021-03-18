#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dgraph;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Data {
  find_person: Vec<FindPerson>,
}

#[derive(Serialize, Deserialize)]
struct FindPerson {
  uid: Option<String>,
  name: Option<String>,
  age: Option<i32>,
  number_of_friends: Option<i32>,
  friend: Option<Vec<Friend>>,
}

#[derive(Serialize, Deserialize)]
struct Friend {
  uid: Option<String>,
  name: Option<String>,
  age: Option<i32>,
}

#[get("/<name>")]
fn index(name: String) -> Json<Vec<FindPerson>> {
  let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:9080"));

  let q = r#"query find_person($a: string) {
    find_person(func: eq(name, $a)) {
      uid
      name
      age
      number_of_friends : count(friend)
      friend {
        uid
        name
        age
      }
    }
  }"#;

  let mut vars = HashMap::new();
  vars.insert("$a".to_string(), name);

  let resp = dgraph
    .new_readonly_txn()
    .query_with_vars(q, vars)
    .expect("Dgraph query");
  let data: Data = serde_json::from_slice(&resp.json).expect("parsing");
  Json(data.find_person)
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
