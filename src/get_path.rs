use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_xdg_or_default(xdg_variant: String, path: String) -> String {
  std::env::var(
    "XDG".to_owned() + &xdg_variant.to_ascii_uppercase() + "_HOME"
  ).unwrap_or_else(
    |_ /* compiler told me to but not sure why i'm even getting an arg here*/| std::env::var("HOME").unwrap() + "/." + &xdg_variant
  ) + "/" + &path
}
pub fn get_cache_location(cache_type: String, requests: &Vec<String>) -> String {
  let mut hasher = DefaultHasher::new();
  let hash = requests.hash(&mut hasher);
  get_xdg_or_default("cache".to_string(),"tenki/".to_string()+&cache_type+"_" + hash+".json") 
}
pub fn get_config_location() -> String {
  get_xdg_or_default("config".to_string(),"tenki_config.yaml".to_string())
}
