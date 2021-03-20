#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dgraph;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

#[derive(Serialize, Deserialize)]
struct PeopleList {
  people: Vec<FindPerson>,
}

#[get("/person/<uid>")]
fn find(uid: String) -> Template {
  let person = get_person(uid);
  Template::render("person", &person[0])
}

#[get("/")]
fn index() -> Template {
  let people = get_people();
  Template::render("index", people)
}

#[get("/json/<name>")]
fn json_find(name: String) -> Json<Vec<FindPerson>> {
  Json(get_person(name))
}

#[get("/json/people")]
fn json_people() -> Json<Vec<FindPerson>> {
  Json(get_people().people)
}

#[get("/public/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("public/").join(file)).ok()
}

fn get_person(uid: String) -> Vec<FindPerson> {
  let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:9080"));

  let q = r#"query find_person($a: string) {
    find_person(func: uid($a)) {
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
  vars.insert("$a".to_string(), uid);

  let resp = dgraph
    .new_readonly_txn()
    .query_with_vars(q, vars)
    .expect("Dgraph query");
  let data: Data = serde_json::from_slice(&resp.json).expect("parsing");
  data.find_person
}

fn get_people() -> PeopleList {
  let dgraph = make_dgraph!(dgraph::new_dgraph_client("localhost:9080"));

  let q = r#"{
    find_person(func: type(Person)) {
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

  let resp = dgraph.new_readonly_txn().query(q).expect("Dgraph query");
  let data: Data = serde_json::from_slice(&resp.json).expect("parsing");
  PeopleList {
    people: data.find_person,
  }
}

fn main() {
  rocket::ignite()
    .attach(Template::fairing())
    .mount("/", routes![find, index, files, json_find, json_people])
    .launch();
}
