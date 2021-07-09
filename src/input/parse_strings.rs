mod parse_time;

use crate::weather_items::MetricType;
use crate::error_strings::ErrorStrings;
use strum::EnumMessage;
pub use parse_time::parse_time as parse_time;
use chrono;
use std::str::FromStr;


pub fn parse_metric_vector(metric_string: String) -> Vec<MetricType>{
  let mut metric_type_vector = Vec::new();
  for metric_string in metric_string.split(","){
      let metric_enum_equivalent = MetricType::from_str(metric_string);
      if metric_enum_equivalent.is_ok() {metric_type_vector.push(metric_enum_equivalent.unwrap())}
  }
  metric_type_vector
}

pub fn parse_location_list(location_list_string: String) -> Vec<String>{
  location_list_string.split(":").collect::<Vec<&str>>().into_iter().map(|item| String::from(item)).collect()
}
pub fn parse_significant_figures(significant_figure_string: String) -> u8{
  significant_figure_string.parse().expect(&(ErrorStrings::NotAParsableNumber.get_message().unwrap().to_owned() + "significant figures"))
}
pub fn parse_cache_duration(cache_duration_string: String)-> chrono::Duration{
  chrono::Duration::minutes(cache_duration_string.parse().expect(&(ErrorStrings::NotAParsableNumber.get_message().unwrap().to_owned() +  "cache duration")))
}