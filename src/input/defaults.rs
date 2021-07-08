use crate::prog_options::ProgOptions;
use crate::weather_items::MetricType;
use chrono;

pub fn get_base_defaults() -> ProgOptions{
  ProgOptions{
    time_list: vec![chrono::Local::now()],
    location_list: vec![String::from("Tokyo")],
    // api: Box<dyn api::Query>,
    human_readable: true,
    significant_figures: 1,
    emoji: true,
    text: true,
    week_starts_sat: false,
    week_starts_sun: false,
    labeled_columns: false,
    graph: Vec::new(),
    cache_duration: chrono::Duration::hours(1),
    metrics: vec![MetricType::Precipitation, MetricType::TemperatureCur]
  }
}  
