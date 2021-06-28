use crate::{ProgOptions, MetricType};
use self::{error_strings, command_line, defaults, config};
use crate::api;
use std::str::FromStr;



fn parse_duration(duration_string: &str) -> Option<chrono::Duration>{
  let time_string: String = String::from(duration_string).chars().filter(|c| c.is_digit(10)).collect();
  let time: i64 = time_string.parse().unwrap(); 
  if duration_string.ends_with("h") {
    Some(chrono::Duration::hours(time))
  } else if duration_string.ends_with("d") { 
    Some(chrono::Duration::days(time))
  } else {
    None 
  }
}

fn get_step_or_default(putative_step: Option<&str>, start_string: &str, stop_string: &str) -> chrono::Duration{
  putative_step.map_or_else(
    || {
      let hour_step = start_string.contains('h') || stop_string.contains('h');
      if hour_step {
        chrono::Duration::hours(1)
      } else { 
        chrono::Duration::days(1)
      }
    }, |step_string| parse_duration(step_string).unwrap() )
}

fn parse_relative_to_current_date_time (duration_string: &str) -> chrono::DateTime<chrono::Local>{
  chrono::Local::now().checked_add_signed(parse_duration(duration_string).unwrap()).unwrap()
}

fn get_vec_of_days(start: chrono::DateTime<chrono::Local>, end: chrono::DateTime<chrono::Local>) -> Vec<chrono::DateTime<chrono::Local>>{
  let mut week_vec = vec![];
  let mut current_day = start;
  while current_day != end {
    week_vec.push(current_day);
    current_day = current_day.check_add_signed(chrono::Duration::days(1)).unwrap();
  }
  week_vec
}
fn get_date_based_on_weekday(weekday: chrono::Weekday, week_offset: u8) -> chrono::DateTime<chrono::Local>{
  chrono::Local.from_local_datetime(NaiveDate::from_isoywd(chrono::offset::Local::now().year(), chrono::offset::Local::now().week() + week_offset, weekday)
}
fn parse_keywords(keyword_string: &str) ->  Vec<chrono::DateTime<chrono::Local>>{
  
  match keyword_string {
    "today" => vec![chrono::Local::now()],
    &_ => vec![]
    "week" => get_vec_of_days(chrono::Local::now(), get_date_based_on_weekday(chrono::Weekday::Sun, 0),
    "weekend" => get_vec_of_days(get_date_based_on_weekday(chrono::Weekday::Sat, 0), get_date_based_on_weekday(chrono::Weekday::Sat, 0).check_add_signed(chrono::Duration::days(1).unwrap()))
    "next week" => get_vec_of_days(get_date_based_on_weekday(chrono::Weekday::Mon, 1), get_date_based_on_weekday(chrono::Weekday::Mon, 1).check_add_signed(chrono::Duration::days(6).unwrap()))
     &_ => panic!(error_strings.NoSuchDateString) 
  }
}

fn parse_time(time_string: &str) -> Vec<chrono::DateTime<chrono::Local>>{
  let mut time_components = time_string.split(":");
  let first_component = time_components.next().unwrap();
  match time_components.next(){
    None => {
      match parse_duration(first_component){
        None => parse_keywords(first_component),
        Some(duration) => vec![chrono::Local::now().checked_add_signed(duration).unwrap()]
      }
    }, Some(stop_string) => {
      let start = parse_relative_to_current_date_time(first_component);
      let step = get_step_or_default(time_components.next(), first_component, stop_string);
      let stop = parse_relative_to_current_date_time(stop_string);
      let mut current_step: chrono::DateTime<chrono::Local> = start;
      let mut return_vec: Vec<chrono::DateTime<chrono::Local>> = vec![];
      while stop > current_step{
        return_vec.push(current_step);
        current_step = current_step.checked_add_signed(step).unwrap();
      }
      return_vec
    }
  }
}

fn parse_metric_vector(metric_string: &str) -> Vec<MetricType>{
    let mut metric_type_vector = Vec::new();
    for metric_string in metric_string.split(","){
      let metric_enum_equivalent = MetricType::from_str(metric_string);
      if metric_enum_equivalent.is_ok() {metric_type_vector.push(metric_enum_equivalent.unwrap())}
    }
    metric_type_vector
  }

fn check_clap_boolean_flags(flag_names: [&str; 4], &mut clap_matches: clap::ArgMatches<'static>) -> [bool; 4]{
  flag_names.into_iter.map(|flag_name| clap_matches.is_present(flag_name));
}

fn parse_location_list(location_list_string: String) -> Vec<&str>{
  location_list_string.split(":").collect()).into_iter().map(|item| String::from(item)).collect();
}
fn parse_significant_figures(significant_figure_string: String) -> u8{
  significant_figure_string.parse().expect(ErrorStrings.NotAParsableNumber.get_message().unwrap() + "significant figures")
}
fn parse_cache_duration(cache_duration_string: String)->{
  chrono::Duration::minutes(cache_duration_string.parse().expect(ErrorStrings.NotAParsableNumber.get_message().unwrap() +  "cache duration"))
}

fn replace_with_config(config: ConfigView, options: ProgOptions) -> ProgOptions{
  options.location_list = config.value_of("location_list").map_or(options.location_list, parse_location_list);
  options.time_list = config.value_of("time_list").map_or(options.time_list, parse_time);
  options.significant_figures = config.value_of("significant_figures").map_or(options.significant_figures, parse_significant_figures);
  options.graph = config.value_of("graph").map_or(options.graph, parse_metric_vector);
  options.cache_duration = config.value_of("cache_duration").map_or(options.cache_duration, parse_cache_duration); 
  options.metrics = config.value_of("metrics").map_or(options.metrics, parse_metric_vector);
}
fn replace_with_arguments(clap_matches: clap::ArgMatches<'static>, options: ProgOptions) -> ProgOptions{

  [options.human_readable, options.emoji, options.text, options.week_starts_sat, options.week_starts_sun, options.labeled_columns] = check_clap_boolean_flags(["emoji", "text", "human_readable", "foo", "week_starts_sat", "week_starts_sun", "labeled_columns"]);

  options.location_list = clap_matches.value_of("location_list").map_or(options.location_list, parse_location_list);
  options.time_list = clap_matches.value_of("time_list").map_or(options.time_list, parse_time);
  options.significant_figures = clap_matches.value_of("significant_figures").map_or(options.significant_figures, parse_significant_figures);
  options.graph = clap_matches.value_of("graph").map_or(options.graph, parse_metric_vector);
  options.cache_duration = clap_matches.value_of("cache_duration").map_or(options.cache_duration, parse_cache_duration); 
  options.metrics = clap_matches.value_of("metrics").map_or(options.metrics, parse_metric_vector);

  options.api =  clap_matches.value_of("api").map_or(options.api, |api_str| api::get_api(api_str).expect(ErrorStrings.NoSuchApi.get_message().unwrap())); 

}

pub fn parse_matches_into_options(clap_matches: clap::ArgMatches<'static>) -> ProgOptions{
  let mut options = defaults::get_base_defaults();
  options = config::get_config_view().map_or(options, |config| replace_with_config(config, &mut options)); 
  options = replace_with_arguments(clap_matches, &mut options);
  options
} 

