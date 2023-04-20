//use rocket::request::Form;
use rocket::fs::TempFile;

//* thread = UID of thread being commented to
#[derive(FromForm)]
pub struct NewComment<'f> {
  pub thread: String,
  pub attachment: TempFile<'f>,
  pub content: String,
}

#[derive(FromForm)]
pub struct NewThread<'f> {
  pub title: String,
  pub attachment: TempFile<'f>,
  pub content: String,
}
