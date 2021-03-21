//use rocket::request::Form;

#[derive(FromForm)]
pub struct NewComment {
  pub thread: String,
  pub attachment: String,
  pub content: String,
}
