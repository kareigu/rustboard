pub fn get_curr_timestamp() -> String {
  chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn parse_text_from_u16(u16_str: Option<String>) -> Option<String> {
  if let Some(content) = u16_str {
    let vec_u16: Result<Vec<u16>, std::num::ParseIntError> = content
      .split_whitespace()
      .map(|token| token.parse())
      .collect();

    let utf = match vec_u16 {
      Ok(v) => String::from_utf16(&v),
      Err(_e) => Ok("Error loading text".to_string()),
    };

    let parsed_content = match utf {
      Ok(s) => s,
      Err(_e) => "Error parsing text".to_string(),
    };

    Some(parsed_content)
  } else {
    None
  }
}

pub fn encode_text_in_u16(s: &String) -> String {
  let mut o = "".to_string();

  for c in s.encode_utf16() {
    o.push_str(format!("{} ", c).as_str());
  }
  o
}
