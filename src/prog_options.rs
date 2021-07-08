use chrono;
use crate::weather_items;

#[derive(Debug)]
pub struct ProgOptions{
  pub time_list: Vec<chrono::DateTime<chrono::Local>>,
  pub location_list: Vec<String>,
  // api: Box<dyn api::Query>,
  pub human_readable: bool,
  pub significant_figures: u8,
  pub emoji: bool,
  pub text: bool,
  pub week_starts_sat: bool,
  pub week_starts_sun: bool,
  pub labeled_columns: bool,
  pub graph:  Vec<weather_items::MetricType>,
  pub cache_duration: chrono::Duration,
  pub metrics: Vec<weather_items::MetricType>
}