use crate::db::types::Attachment;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::data::TempFile;
use regex::Regex;

pub async fn write_attachment(file: &mut TempFile<'_>) -> std::io::Result<Option<Attachment>> {
  if file.len() > 0 {
    let content_type = file.content_type().unwrap().to_string();
    let file_slug: String = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(14)
      .map(char::from)
      .collect();
    let filename = format!("{}.{}", file_slug, content_type.split("/").last().unwrap(),);
    let new_attachment = Attachment {
      filename,
      content_type,
    };
    let path = format!("./files/{}", new_attachment.filename);
    file.persist_to("/tmp/temp.png").await?;
    std::fs::copy("/tmp/temp.png", path).expect("Failed copying file");
    std::fs::remove_file("/tmp/temp.png").expect("Couldn't remove file");
    Ok(Some(new_attachment))
  } else {
    Ok(None)
  }
}


pub fn check_uid_validity(s: &String) -> bool {
  let re = Regex::new(r"^0x([A-z]|[0-9]){1,}$").unwrap();
  re.is_match(s)
}
