use std::io::{ErrorKind, Error, Result};

use crate::db::types::Attachment;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::data::TempFile;

//* Write file to the public folder
//* All files have their name replaced with a randomly generated
//* 14 character alphanumeric string
pub async fn write_attachment(file: &mut TempFile<'_>) -> Result<Option<Attachment>> {
  if file.len() > 0 {
    if let Some(c) = file.content_type() {
      let content_type = c.to_string();
      let file_slug: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(14)
        .map(char::from)
        .collect();

      let file_extension = match content_type.split("/").last() {
        None => "png",
        Some(s) => {
          if s == "mpeg" {
            "mp3"
          } else {
            s
          }
        },
      };
      
      let filename = format!("{}.{}", file_slug, file_extension);
      let new_attachment = Attachment {
        filename,
        content_type,
      };

      match file.persist_to("/tmp/temp.png").await {
        Ok(_) => {
          let path = format!("./files/{}", new_attachment.filename);
          if let Err(e) = std::fs::copy("/tmp/temp.png", path) { Err(e) }
          else {
            if let Err(e) = std::fs::remove_file("/tmp/temp.png") { Err(e) }
            else {
              Ok(Some(new_attachment))
            }
          }
        },
        Err(e) => Err(e),
      }
    } else {
      Err(Error::new(ErrorKind::Other, "No content-type"))
    }
  } else {
    Ok(None)
  }
}
