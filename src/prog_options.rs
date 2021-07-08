use chrono;

#[derive(Debug)]
pub struct ProgOptions{
  time_list: Vec<chrono::DateTime<chrono::Local>>,
  location_list: Vec<String>,
  // api: Box<dyn api::Query>,
  human_readable: bool,
  significant_figures: i8,
  emoji: bool,
  text: bool,
  week_starts_sat: bool,
  week_starts_sun: bool,
  labeled_columns: bool,
  graph:  Vec<weather_item::MetricType>,
  cache_duration: chrono::Duration,
  metrics: Vec<weather_item::MetricType>
}