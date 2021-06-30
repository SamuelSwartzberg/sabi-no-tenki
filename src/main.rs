 
use chrono;
mod api;
mod weather_item;

#[derive(Debug)]
pub struct ProgOptions{
  time_list: Vec<chrono::DateTime<chrono::Local>>,
  location_list: Vec<String>,
  api: api::metaweather_query::MetaweatherQuery,
  human_readable: bool,
  significant_figures: u8,
  emoji: bool,
  text: bool,
  week_starts_sat: bool,
  week_starts_sun: bool,
  labeled_columns: bool,
  graph: Vec<weather_item::MetricType>,
  cache_duration: chrono::Duration,
  metrics: Vec<weather_item::MetricType>
}

fn main() {
  // functions returning values should be pure functions, outside of throwing errors
  let options = ProgOptions{
    time_list: vec![chrono::Local::now(), chrono::Local::now().checked_add_signed(chrono::Duration::days(1)).unwrap(), chrono::Local::now().checked_add_signed(chrono::Duration::days(2)).unwrap()],
    location_list: vec!["Tokyo".to_string(), "Berlin".to_string()],
    api: api::metaweather_query::MetaweatherQuery{},
    human_readable: true,
    significant_figures: 2,
    emoji: true,
    text: true,
    week_starts_sat: false,
    week_starts_sun: false,
    labeled_columns: false,
    graph: vec![weather_item::MetricType::Precipitation],
    cache_duration: chrono::Duration::hours(1),
    metrics: vec![weather_item::MetricType::Precipitation, weather_item::MetricType::TemperatureCur] 
  };
  // let request = options.api.build_request(&options);
  // let result = http_request::get_result_from_request(request);
  // cache::cache_result(&result, options.cache_duration);
  // let weather_parsed_result = options.api.parse_result(&result);
}
