use crate::get_path;
use serde_yaml;
use serde::de::Deserialize;

pub struct ConfigView{
  config_map: serde_yaml::Value
}
impl ConfigView{
  pub fn value_of(&self, arg: &str) -> Option<String>{
    self.config_map.get(arg).and_then(|value| {
      match value.as_bool() {
        Some(bool_val) => Some(bool_val.to_string()),
        None => match value.as_str() {
          Some(str_val) => Some(String::from(str_val)),
          None => match value.as_i64() {
            Some(i64_val) => Some(i64_val.to_string()),
            None => None
          }
        }
      }
    })
  }
}
pub fn get_config_view() -> Option<ConfigView>{
  let yaml_string = std::fs::read_to_string(get_path::get_config_location()?).ok()?;
  let deserializer = serde_yaml::Deserializer::from_str(&yaml_string);
  let toplevel_yaml_item =  serde_yaml::Value::deserialize(deserializer).ok()?;
  if !toplevel_yaml_item.is_mapping() {
    None
  } else {
    Some(ConfigView{
      config_map: toplevel_yaml_item
    })
  }
}


