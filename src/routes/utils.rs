const CREATE_POST_CODES_C: usize = 4;
const CREATE_POST_CODES: [&str; CREATE_POST_CODES_C] = 
  ["Cannot have a post with no content", 
  "Unknown error",
  "Error handling attachment",
  "Database error"];

pub fn get_create_post_err_msg(e: Option<usize>) -> Option<String> {
  match e {
    Some(i) => if i >= CREATE_POST_CODES_C { None } else { Some(CREATE_POST_CODES[i].to_string()) },
    None => None
  }
}