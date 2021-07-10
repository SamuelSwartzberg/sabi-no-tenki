use crate::get_path;
use serde::{Deserialize, Serialize};
use chrono::{Local};
use crate::utils::system_time_to_date_time;

pub fn get_result_from_cache(&cache_duration: chrono::Duration, suffix: String)-> Option<Vec<String>>{ 
  if let Ok(metadata) = std::fs::metadata(get_path::get_cache_location(suffix)){
    if let Ok(time) = metadata.created(){
      let file_creation_time = system_time_to_date_time(time);
      if file_creation_time.checked_add_signed(cache_duration) > chrono::Local::now() {
        let cache_string = std::fs::read_to_string(get_path::get_cache_location()).ok();
        Some(serde_json::from_str(cache_string).unwrap().as_array().unwrap())
      } else { None }
    } else {None }
  } else { None }
} 
 
pub fn cache_result(result: &Vec<String>, suffix: String) {
  std::fs::write(get_path::get_cache_location(suffix), serde_json::to_string(result).unwrap());
}
