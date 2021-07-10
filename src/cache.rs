use crate::get_path;
/*use serde::{Deserialize, Serialize};*/
/*use chrono::{Local};*/
use crate::utils::system_time_to_date_time;

pub fn get_result_from_cache(cache_duration: &chrono::Duration, cache_type: &str, request: &Vec<String>)-> Option<Vec<String>>{ 
  if let Ok(metadata) = std::fs::metadata(get_path::get_cache_location(cache_type, &request)){
    if let Ok(time) = metadata.created(){
      let file_creation_time = system_time_to_date_time(time);
      if file_creation_time.checked_add_signed(cache_duration.clone()).unwrap() > chrono::Local::now() {
        let cache_string = std::fs::read_to_string(get_path::get_cache_location(cache_type, &request)).unwrap();
        Some(serde_json::from_str::<serde_json::Value>(&cache_string).unwrap().as_array().unwrap().into_iter().map(|elem| elem.to_string()).collect())
      } else { None }
    } else {None }
  } else { None }
} 
 
pub fn cache_result(result: &Vec<String>, cache_type: &str, request: &Vec<String>) {
  std::fs::write(get_path::get_cache_location(cache_type, &request), serde_json::to_string(result).unwrap());
}
