

//   metaweather, //https://www.metaweather.com/api/#locationsearch
//  Weather-related response fields
// weather_state_name
  // Snow
  // Sleet
  // Hail
  // Thunderstorm
  // Heavy Rain
  // Light Rain
  // Showers
  // Heavy Cloud
  // Light Cloud
  // Clear
// weather_state_abbr
// wind_speed	
// wind_direction	
// wind_direction_compass	
// (min|max|the)_temp	
// air_pressure	
// humidity	
// visibility
// predictability	
#[derive(Debug)]
pub struct MetaweatherQuery{
//  generic_metric_names_map: HashMap<MetricType, &str>
}
//
// impl Query for MetaweatherQuery{
//  fn build_request(options: &ProgOptions ){};
//  fn parse_result() -> Vec<WeatherItem>{};
// Igonore for now: fn get_names(&self) -> Vec<&str>{
//    vec!["metaweather", "mw"]     
//  }
//}
