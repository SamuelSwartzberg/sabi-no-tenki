use crate::{ProgOptions, MetricType};
use chrono;

fn get_base_defaults() -> ProgOptions{
  ProgOptions{
    time_list: vec![chrono::Local::now()],
    location_list: vec!["Tokyo"],
    // api: Box<dyn api::Query>,
    human_readable: true,
    significant_figures: 1,
    emoji: true,
    text: true,
    graph: Vec::new(),
    cache_duration: chrono::Duration::hours(1),
    metrics: vec![MetricType::Precipitation, MetricType::TemperatureCur]
  }
}  
