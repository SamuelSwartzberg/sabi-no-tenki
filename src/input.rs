use clap::{Arg, App /*, SubCommand*/};
use crate::{ProgOptions, MetricType};
use crate::api;
use std::str::FromStr;


pub fn get_command_line_input() -> clap::ArgMatches<'static>  { //possibly remove static lifetimes once I become clear what the lifetimes of App are
    return App::new("Sabi no Tenki")
    .version("0.0")
    .author("Sam S. <me@samswartzberg.com>")
    .help("A terminal command line weather client with fine-grained options and a pretentious and possibly incorrect japanese name.")
    .arg(Arg::with_name("location")
        .short("l")
        .long("location")
        .value_name("LOCATION_LIST")
        .help("Use the specified location. Best specified in options file for default location. List is separated by colons (:)")
        .takes_value(true))
    .arg(Arg::with_name("api")
        .long("api")
        .value_name("API")
        .help("Use specified api. Not all requested metrics are available for all apis.")
        .takes_value(true))
    .arg(Arg::with_name("human-readable")
        .short("h")
        .long("human-readable")
        .help("Use human-readable text instead of CSV-like syntax"))
    .arg(Arg::with_name("ascii-image")
        .short("a")
        .long("ascii-image")
        .help("Include an ascii image for every requested unit"))
    .arg(Arg::with_name("graph")
        .short("g")
        .long("graph")
        .help("Show an ascii graph for the requested metrics")
        .value_name("METRICS")
        .takes_value(true))
    .arg(Arg::with_name("cache-duration")
        .long("cache-duration")
        .help("Specify the duration to cache previous results (in minutes)")
        .value_name("DURATION")
        .takes_value(true))
    .arg(Arg::with_name("significant-figures")
        .long("significant-figures")
        .help("give this amount of significant figures (e.g. --significant-figures 1 -> 25.1 C)")
        .value_name("FIGURES")
        .takes_value(true))
    .arg(Arg::with_name("metrics")
        .long("metrics")
        .help("Specify a list of metrics you would like to recieve, as a non-spaced comma-separated list. Not all requested metrics are available for all apis.")
        .value_name("METRIC-LIST")
        .takes_value(true))
    .arg(Arg::with_name("emoji")
        .long("emoji")
        .help("Show things such as current weather as emoji (e.g. '🌧️'). Can be combined with --text if both are desired (e.g. '🌧️ Rainy')."))
    .arg(Arg::with_name("text")
        .long("text")
        .help("Show things such as current weather as text (e.g. 'Rainy'). Can be combined with --emoji if both are desired (e.g. '🌧️ Rainy')."))
    .arg(Arg::with_name("time")
        .help("The time span to fetch weather for:
            time = [start[:end][:step]]] | shorthand-values
            start = time-value
            end = time-value
            step = time-value
            time-value = time,unit
            time = 0...23
            unit = 'd'|'h'
            shorthand-values = 'today'|'week'|'weekend'|'next-week'

            no value - now
            1h - in one hour
            5d - in 5 days
            2w - in two weeks
            1h:10h - in the 9 hours starting one hour from now
            1h:10h:3h - in the in the 9 hours starting one hour from now every three hours
            :6d:2d - in the next six days, every second day


            Leaving out step is usually fine - if you specify a h value for any of the values, 
            it will presume hour-based stepping, otherwise it will presume day-based stepping.")
        .index(1))
    .get_matches();
}
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

fn parse_keywords(keyword_string: &str) ->  Vec<chrono::DateTime<chrono::Local>>{
  match keyword_string {
    "today" => vec![chrono::Local::now()],
    &_ => vec![]
   // "week",
   // "weekend",
   // "next week"
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

fn get_metric_vector(clap_parameter_option: Option<&str>) -> Vec<MetricType>{
    let mut metric_type_vector = Vec::new();
    for metric_string in clap_parameter_option.map_or("".split(","), |graph_string| graph_string.split(",")){
      let metric_enum_equivalent = MetricType::from_str(metric_string);
      if metric_enum_equivalent.is_ok() {metric_type_vector.push(metric_enum_equivalent.unwrap())}
    }
    metric_type_vector
  }
pub fn parse_matches_into_options(clap_matches: clap::ArgMatches) -> ProgOptions{
  let location_list = clap_matches.value_of("location").map_or(Vec::<&str>::new(), |location_list_string| location_list_string.split(":").collect());
  let location_list = location_list.into_iter().map(|item| String::from(item)).collect();
  let time_list = clap_matches.value_of("time").map_or(vec![chrono::Local::now()], |time_string| parse_time(time_string));
  let api = api::get_api(clap_matches.value_of("api").unwrap_or("not sure what we want to do when no api specified")).unwrap(); 
  let human_readable = clap_matches.is_present("human-readable");
  let significant_figures = clap_matches.value_of("significant_figures").map_or(0,|significant_figure_string| significant_figure_string.parse().unwrap());
  let emoji = clap_matches.is_present("emoji");
  let text = clap_matches.is_present("text");
  let graph = get_metric_vector(clap_matches.value_of("graph"));
  let cache_duration = clap_matches.value_of("cache-duration").map_or(chrono::Duration::minutes(60), |cache_duration_string| chrono::Duration::minutes(cache_duration_string.parse().unwrap())); 
  let metrics = get_metric_vector(clap_matches.value_of("metrics"));
  return ProgOptions{
    location_list,
    time_list,
    api,
    human_readable,
    significant_figures,
    emoji,
    text,
    graph,
    cache_duration,
    metrics
  }
} 

