use chrono;
mod input;
mod api;
extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(EnumString, Display)] 
#[strum(ascii_case_insensitive)]
enum MetricType{
  WeatherType,
  WindSpeed,
  WindDirection,
  TemperatureCur,
  TemperatureMin,
  TemperatureMax,
  Humidity,
  Pressure,
  Precipitation,
  UvIndex,
  AirQuality,
  Visibility,
  Predictability,
  CloudCover,
  HeatIndex,
  Dewpoint,
  WindChill,
  WindGust,
  FeelsLike,
  ChanceOfRain,
  ChanceOfRemainingDry,
  ChanceOfWindy,
  ChanceOfOvercast,
  ChanceOfSunshine,
  ChanceOfFrost,
  ChanceOfHighTemp,
  ChanceOfFog,
  ChanceOfSnow,
  ChanceOfThunder
}

struct ProgOptions{
  time_list: Vec<chrono::DateTime<chrono::Local>>,
  location_list: Vec<String>,
  api: Box<dyn api::Query>,
  human_readable: bool,
  significant_figures: i8,
  emoji: bool,
  text: bool,
  graph: Vec<MetricType>,
  cache_duration: chrono::Duration,
  metrics: Vec<MetricType>
}

struct WeatherItem{ 
  date: chrono::DateTime<chrono::Local>, 
  location: String, 
  metrics: Vec<Metric> 
} 

struct Metric{
  type_of: MetricType,
  value: Box<dyn std::fmt::Display>
}

fn main() {
  // get values from config file, not sure how. It would be nice if we could feed the config file into clap.rs somehow
  // github issuses mentioning this:
  // https://github.com/clap-rs/clap/issues/1693
  // https://github.com/clap-rs/clap/issues/748
  // https://github.com/clap-rs/clap/issues/251

  // functions returning values should be pure functions, outside of throwing errors
  let _matches = input::get_command_line_input();
  // let options = input::parse_matches_into_options(matches);
  // let request = options.api.build_request(&options);
  // let result = http_request::get_result_from_request(request);
  // cache::cache_result(&result, options.cache_duration);
  // let weather_parsed_result = options.api.parse_result(&result);
  // let output = output_generator::generate_output(&weather_parsed_result, &options, get_shell_specs());
  // println!(output);
}
