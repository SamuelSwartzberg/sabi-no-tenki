mod config;
mod defaults;
mod parse_strings;

use crate::prog_options::{ProgOptions, WeekStarts};
// use crate::api;
use parse_strings::{parse_location_list, parse_time, parse_metric_vector, parse_significant_figures, parse_cache_duration};
use config::ConfigView;

fn check_clap_boolean_flags(flag_names: Vec<&str>, clap_matches: &clap::ArgMatches) -> Vec<bool>{
  flag_names.iter().map(|flag_name| clap_matches.is_present(flag_name)).collect()
}

fn replace_with_arguments_required_for_input_parsing_itself(clap_matches: &clap::ArgMatches, options: &mut ProgOptions){
  let bool_flags = check_clap_boolean_flags(vec![ "week_starts_sat", "week_starts_sun"], &clap_matches);
  if bool_flags[0] {options.week_starts = WeekStarts::Sat}
  else if bool_flags[1] {options.week_starts = WeekStarts::Sun}
}


fn replace_with_config(config: ConfigView, options: &mut ProgOptions){
  if let Some(res) = config.value_of("location_list") {options.location_list = parse_location_list(res)};
  if let Some(res) = config.value_of("time_list") {options.time_list = parse_time(res, &options.week_starts)};
  if let Some(res) = config.value_of("significant_figures") {options.significant_figures = parse_significant_figures(res)};
  if let Some(res) = config.value_of("graph") {options.graph = parse_metric_vector(res)};
  if let Some(res) = config.value_of("cache_duration") {options.cache_duration = parse_cache_duration(res)}; 
  if let Some(res) = config.value_of("metrics") {options.metrics = parse_metric_vector(res)};
  
}
fn replace_with_arguments(clap_matches: &clap::ArgMatches, options: &mut ProgOptions){
  let bool_flags = check_clap_boolean_flags(vec!["emoji", "text", "human_readable", "labeled_columns"], &clap_matches);
  options.emoji = bool_flags[0]; options.text = bool_flags[1]; options.human_readable = bool_flags[2]; options.labeled_columns = bool_flags[3];
  if let Some(res) = clap_matches.value_of("location_list").map(String::from) { options.location_list = parse_location_list(res)};
  if let Some(res) = clap_matches.value_of("time_list").map(String::from) {options.time_list = parse_time(res, &options.week_starts)};
  if let Some(res) = clap_matches.value_of("significant_figures").map(String::from) {options.significant_figures = parse_significant_figures(res)};
  if let Some(res) = clap_matches.value_of("graph").map(String::from) {options.graph = parse_metric_vector(res)};
  if let Some(res) = clap_matches.value_of("cache_duration").map(String::from) {options.cache_duration = parse_cache_duration(res)}; 
  if let Some(res) = clap_matches.value_of("metrics").map(String::from) {options.metrics = parse_metric_vector(res)};

  //options.api =  clap_matches.value_of("api").map_or(options.api, |api_str| api::get_api(api_str).expect(ErrorStrings::NoSuchApi.get_message().unwrap())); 
}

pub fn parse_matches_into_options(clap_matches: clap::ArgMatches) -> ProgOptions{
  let mut options = defaults::get_base_defaults();
  replace_with_arguments_required_for_input_parsing_itself(&clap_matches, &mut options);
  if let Some(config) = config::get_config_view() { replace_with_config(config, &mut options)}; 
  replace_with_arguments(&clap_matches, &mut options);
  options
} 

