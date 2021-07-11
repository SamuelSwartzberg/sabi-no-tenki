use crate::get_path;
/*use serde::{Deserialize, Serialize};*/
/*use chrono::{Local};*/
use serde_json;
use crate::utils::system_time_to_date_time;

#[allow(unused_must_use)]
pub fn get_result_from_cache(cache_duration: &chrono::Duration, cache_type: &str, requests: &Vec<String>)-> Option<Vec<serde_json::Value>>{ 
  match std::fs::metadata(get_path::get_cache_full_path(cache_type, &requests)?){
    Ok(metadata) => {
      match metadata.created(){
        Ok(time) => {
          let file_creation_time = system_time_to_date_time(time);
          if file_creation_time.checked_add_signed(cache_duration.clone())? > chrono::Local::now() {
            let cache_string = std::fs::read_to_string(get_path::get_cache_full_path(cache_type, &requests)?).ok()?;
            Some(serde_json::from_str::<serde_json::Value>(&cache_string).ok()?.as_array()?.clone())
          } else { 
            std::fs::remove_file(get_path::get_cache_full_path(cache_type, &requests)?);
            None }
        }
        Err(error) => {eprintln!("{:#?}", error); None}
      }
    }
    Err(error) => {eprintln!("{:#?}", error); None}
  }
} 
 
#[allow(unused_must_use)]
pub fn cache_result(result: &Vec<serde_json::Value>, cache_type: &str, requests: &Vec<String>) -> Option<()>{
  std::fs::create_dir(get_path::get_cache_path()?);
  std::fs::remove_file(get_path::get_cache_full_path(cache_type, &requests)?); //remove file so file creation time will be updated
  std::fs::write(get_path::get_cache_full_path(cache_type, &requests)?, serde_json::to_string(result).ok()?);
  Some(())
}
