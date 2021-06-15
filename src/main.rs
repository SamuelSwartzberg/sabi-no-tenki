use clap::{Arg, App, SubCommand};
use chrono;
use api, input, cache, http_request, output_generator;

enum MetricType{
  weather_type,
  wind_speed,
  wind_direction,
  temperature_cur,
  temperature_min,
  temperature_max,
  humidity,
  pressure,
  precipitation,
  uv_index,
  air_quality,
  visibility,
  predictability,
  feels_like,
  cloud_cover,
  heat_index,
  dewpoint,
  wind_chill,
  wind_gust,
  feels_like,
  chance_of_rain,
  chance_of_remaining_dry,
  chance_of_windy,
  chance_of_overcast,
  chance_of_sunshine,
  chance_of_frost,
  chance_of_high_temp,
  chance_of_fog,
  chance_of_snow,
  chance_of_thunder
}

struct ProgOptions{
  time_list: Vec<chrono::DateTime>,
  location_list: Vec<&str>,
  api: impl api::Query,
  human_readable: bool,
  significant_figures: i8,
  emoji: bool,
  text: bool,
  graph: Vec<MetricType>,
  cache_duration: chrono::Duration,
  metrics: Vec<MetricType>
}

struct WeatherItem{ 
  date: chrono::DateTime, 
  location: &str, 
  metrics: Vec<Metric> 
} 

fn main() {
  // get values from config file, not sure how. It would be nice if we could feed the config file into clap.rs somehow
  // github issuses mentioning this:
  // https://github.com/clap-rs/clap/issues/1693
  // https://github.com/clap-rs/clap/issues/748
  // https://github.com/clap-rs/clap/issues/251

  // functions returning values should be pure functions, outside of throwing errors
  let matches = get_command_line_input();
  let options = parse_matches_into_options(matches);
  let request = options.api.build_request(&options);
  let result = get_result_from_request(request);
  cache_result(&result, options.cache_duration);
  let weather_parsed_result = options.api.parse_result(&result);
  let output = generate_output(&weather_parsed_result, &options, get_shell_specs());
  println!(output);
}
