use crate::get_path;

fn get_result_from_cache(cache_duration: chrono::Duration) -> Option<&str>{ 
  if let Ok(metadata) = std::fs::metadata(get_path::get_cache_location()){
    if let Ok(time) = metadata.created(){
      let file_creation_time = chrono::DateTime<Local>.from(time);
      if file_creation_time.checked_add_signed(cache_duration) > chrono::Local::now() {
        Some(std::fs::read_to_string(get_path::get_cache_location()).ok())
      } else { None }
    } else {None }
  } else { None }
} 
 
fn cache_result(result: &str) {
  std::fs::write(get_path::get_cache_location(), result);
}
