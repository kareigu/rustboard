use std::io::{Error, ErrorKind, Result};

use crate::db::types::Attachment;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::fs::TempFile;

//* Write file to the public folder
//* All files have their name replaced with a randomly generated
//* 14 character alphanumeric string
pub async fn write_attachment(file: &mut TempFile<'_>) -> Result<Option<Attachment>> {
  if file.len() == 0 {
    return Ok(None);
  }

  let c = match file.content_type() {
    Some(c) => c,
    None => return Err(Error::new(ErrorKind::Other, "No content-type")),
  };

  let file_slug: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(14)
    .map(char::from)
    .collect();

  let content_type = c.to_string();
  let file_extension = match content_type.split('/').last() {
    None => "png",
    Some(s) => match s {
      "mpeg" => "mp3",
      _ => s,
    },
  };

  let filename = format!("{}.{}", file_slug, file_extension);
  let new_attachment = Attachment {
    filename,
    content_type,
  };

  file
    .copy_to(format!("./files/{}", new_attachment.filename))
    .await?;

  Ok(Some(new_attachment))
}
