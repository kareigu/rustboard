use crate::db::types::Attachment;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::data::TempFile;

pub async fn write_attachment(file: &mut TempFile<'_>) -> std::io::Result<Attachment> {
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
  Ok(new_attachment)
}
