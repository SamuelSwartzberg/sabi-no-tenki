use crate::get_path;
use serde::{Deserialize, Serialize};

fn get_result_from_cache(cache_duration: chrono::Duration)-> Option<Vec<String>>{ 
  if let Ok(metadata) = std::fs::metadata(get_path::get_cache_location()){
    if let Ok(time) = metadata.created(){
      let file_creation_time = chrono::DateTime<Local>.from(time);
      if file_creation_time.checked_add_signed(cache_duration) > chrono::Local::now() {
        let cache_string = std::fs::read_to_string(get_path::get_cache_location()).ok();
        Some(serde_json::from_str(cache_string).unwrap().as_array().unwrap())
      } else { None }
    } else {None }
  } else { None }
} 
 
fn cache_result(result: Vec<String>) {
  std::fs::write(get_path::get_cache_location(), serde_json::to_string(result).unwrap());
}
