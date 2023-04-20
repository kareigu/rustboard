use regex::Regex;

pub fn check_uid_validity(s: &str) -> bool {
  let re = Regex::new(r"^0x([A-z]|[0-9]){1,}$").unwrap();
  re.is_match(s)
}
