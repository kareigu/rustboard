//use rocket::request::Form;
use rocket::data::TempFile;

#[derive(FromForm)]
pub struct NewComment<'f> {
  pub thread: String,
  pub attachment: TempFile<'f>,
  pub content: String,
}
