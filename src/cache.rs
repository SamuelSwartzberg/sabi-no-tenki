use crate::get_path;
/*use serde::{Deserialize, Serialize};*/
/*use chrono::{Local};*/
use crate::utils::system_time_to_date_time;

pub fn get_result_from_cache(cache_duration: &chrono::Duration, cache_type: &str, requests: &Vec<String>)-> Option<Vec<String>>{ 
  match std::fs::metadata(get_path::get_cache_full_path(cache_type, &requests)){
    Ok(metadata) => {
      match metadata.created(){
        Ok(time) => {
          let file_creation_time = system_time_to_date_time(time);
          if file_creation_time.checked_add_signed(cache_duration.clone()).unwrap() > chrono::Local::now() {
            let cache_string = std::fs::read_to_string(get_path::get_cache_full_path(cache_type, &requests)).unwrap();
            Some(serde_json::from_str::<serde_json::Value>(&cache_string).unwrap().as_array().unwrap().into_iter().map(|elem| elem.to_string()).collect())
          } else { 
            eprintln!("{:#?}", "Cache too old, won't use."); 
            std::fs::remove_file(get_path::get_cache_full_path(cache_type, &requests));
            None }
        }
        Err(error) => {eprintln!("{:#?}", error); None}
      }
    }
    Err(error) => {eprintln!("{:#?}", error); None}
  }
} 
 
pub fn cache_result(result: &Vec<String>, cache_type: &str, requests: &Vec<String>) {
  std::fs::create_dir(get_path::get_cache_path());
  std::fs::remove_file(get_path::get_cache_full_path(cache_type, &requests)); //remove file so file creation time will be updated
  std::fs::write(get_path::get_cache_full_path(cache_type, &requests), serde_json::to_string(result).unwrap());
}
