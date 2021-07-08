fn get_xdg_or_default(xdg_variant: String, path: String) -> String {
  std::env::var("XDG" + xdg_variant.to_ascii_uppercase() + "_HOME").unwrap_or_else(|| std::env::var("HOME").unwrap() + "/." + xdg_variant) + "/" + path
}
pub fn get_cache_location() -> String {
  get_xdg_or_default("cache","tenki.json") 
}
pub fn get_config_location() -> String {
  get_xdg_or_default("config","tenki_config.yaml")
}
