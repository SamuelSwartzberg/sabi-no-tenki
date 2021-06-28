fn get_xdg_or_default(xdg_variant: String, path: String) -> &str {
  std::env::var("XDG" + xdg_variant.to_ascii_uppercase() + "_HOME").unwrap_or_else(|| std::env::var("HOME").unwrap() + "/." + xdg_variant) + "/" + path
}
fn get_cache_location() -> &str {
  get_xdg_or_default("cache","tenki.json") 
}
fn get_config_location() -> &str {
  get_xdg_or_default("config","tenki_config.yaml")
}
