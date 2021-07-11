use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[allow(unused_assignments)]
fn get_xdg_or_default(xdg_variant: String) -> Option<String> {
  let mut output = "".to_owned();
  if let Ok(xdg_var) = std::env::var(
    "XDG".to_owned() + &xdg_variant.to_ascii_uppercase() + "_HOME"
  ) {
    output = xdg_var;
  } else {
    output = std::env::var("HOME").ok()? + "/." + &xdg_variant;
  }
  Some(output + "/")
}
pub fn get_cache_path()-> Option<String>{
  Some(get_xdg_or_default("cache".to_string())?+"tenki/")
}
pub fn get_cache_file_name(cache_type: &str, requests: &Vec<String>) ->  String {
  let mut hasher = DefaultHasher::new();
  requests.hash(&mut hasher);
  let hash = hasher.finish().to_string();
  cache_type.to_owned()+"_" + &hash+".json"
}

pub fn get_cache_full_path(cache_type: &str, requests: &Vec<String>) -> Option<String>{
  Some(get_cache_path()?+&get_cache_file_name(cache_type, &requests))
}
pub fn get_config_location() -> Option<String> {
  Some(get_xdg_or_default("config".to_string())?+"tenki_config.yaml")
}
