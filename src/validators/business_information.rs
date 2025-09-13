pub fn validate_rpps_number(rpps: &str) -> bool {
  let rpps_matching_regex = regex::Regex::new(r"^\d{11}$").unwrap();
  rpps_matching_regex.is_match(rpps)
}

pub fn validate_siret_number(siret: &str) -> bool {
  let siret_number_regex = regex::Regex::new(r"^\d{14}$").unwrap();
  siret_number_regex.is_match(siret)
}
