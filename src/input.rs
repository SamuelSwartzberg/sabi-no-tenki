mod config;
mod defaults;
mod error_strings;
mod parse_strings;

use crate::prog_options::ProgOptions;
// use crate::api;
use parse_strings::{parse_location_list, parse_time, parse_metric_vector, parse_significant_figures, parse_cache_duration};
use config::ConfigView;

fn check_clap_boolean_flags(flag_names: [&str; 4], clap_matches: &mut clap::ArgMatches<'static>) -> [bool; 4]{
  flag_names.into_iter().map(|flag_name| clap_matches.is_present(flag_name));
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

  //options.api =  clap_matches.value_of("api").map_or(options.api, |api_str| api::get_api(api_str).expect(ErrorStrings::NoSuchApi.get_message().unwrap())); 

}

pub fn parse_matches_into_options(clap_matches: clap::ArgMatches<'static>) -> ProgOptions{
  let mut options = defaults::get_base_defaults();
  options = config::get_config_view().map_or(options, |config| replace_with_config(config, &mut options)); 
  options = replace_with_arguments(clap_matches, &mut options);
  options
} 

