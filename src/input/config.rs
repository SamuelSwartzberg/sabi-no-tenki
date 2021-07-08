use crate::get_path;
use serde_yaml;

pub struct ConfigView{
  config_map: serde_yaml::Value
}
impl ConfigView{
  pub fn value_of(&self, arg: &str) -> Option<String>{
    self.get(arg).and_then(|value| value.as_str())
  }
}
pub fn get_config_view() -> Option<ConfigView>{
  let yaml_string = std::fs::read_to_string(get_path::get_config_location())?;
  let deserializer = serde_yaml::Deserializer::from_str(yaml_string)?;
  let toplevel_yaml_item =  serde_yaml::Value::deserialize(deserializer)?;
  if !toplevel_yaml_item.is_mapping() {
    None
  } else {
    Some(ConfigView{
      config_map: toplevel_yaml_item
    })
  }
}


