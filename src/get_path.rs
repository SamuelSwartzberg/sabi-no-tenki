use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_xdg_or_default(xdg_variant: String) -> String {
  std::env::var(
    "XDG".to_owned() + &xdg_variant.to_ascii_uppercase() + "_HOME"
  ).unwrap_or_else(
    |_ /* compiler told me to but not sure why i'm even getting an arg here*/| std::env::var("HOME").unwrap() + "/." + &xdg_variant
  ) + "/"
}
pub fn get_cache_path()->String{
  get_xdg_or_default("cache".to_string())+"tenki/"
}
pub fn get_cache_file_name(cache_type: &str, requests: &Vec<String>) -> String {
  let mut hasher = DefaultHasher::new();
  requests.hash(&mut hasher);
  let hash = hasher.finish().to_string();
  println!("{:#?}", hash);
  cache_type.to_owned()+"_" + &hash+".json"
}

pub fn get_cache_full_path(cache_type: &str, requests: &Vec<String>) -> String{
  get_cache_path()+&get_cache_file_name(cache_type, &requests)
}
pub fn get_config_location() -> String {
  get_xdg_or_default("config".to_string())+"tenki_config.yaml"
}
