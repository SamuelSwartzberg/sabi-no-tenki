use chrono;
use terminal_size;
mod input;
mod api;
extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(EnumString, Debug, Display)] 
#[strum(ascii_case_insensitive)]
pub enum MetricType{
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

pub enum WeatherType{
  #[strum(message="☀️", detailed_message="clear")]
  Clear,
  #[strum(message="⛅", detailed_message="partially cloudy")]
  PartlyCloudy,
  #[strum(message="☁️", detailed_message="cloudy")]
  Cloudy,
  #[strum(message="💨", detailed_message="dust")]
  Dust,
  #[strum(message="🌁", detailed_message="mist")]
  Mist,
  #[strum(message="🌫️", detailed_message="fog")]
  Fog,
  #[strum(message="🌧️", detailed_message=rain"")]
  Rain,
  #[strum(message="🌨️", detailed_message=snow"")]
  Snow,
  #[strum(message="🌨️🏃", detailed_message=snow showers"")]
  SnowShower,
  #[strum(message="", detailed_message="darude - sandstorm")]
  Sandstorm,
  #[strum(message="🌧️🌨️", detailed_messagerain and snow="")]
  RainSnow,
  #[strum(message="🌧️🌨️🏃", detailed_messagerain and snow showers="")]
  RainSnowShower,
  #[strum(message="💦", detailed_message="drizzle")]
  LightRain, //also drizzle
  #[strum(message="🌧️🏃", detailed_message=rain showers"")]
  RainShower,
  #[strum(message="💧❄️", detailed_message="sleet")]
  Sleet,
  #[strum(message="❄️🌧️", detailed_message=freezing rain"")]
  FreezingRain,
  #[strum(message="🤕❄️", detailed_message="hail")]
  Hail,
  #[strum(message="🌩️", detailed_message="thunderstorm")]
  Thunderstorms 
}
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
  graph: Vec<MetricType>,
  cache_duration: chrono::Duration,
  metrics: Vec<MetricType>
}

pub struct WeatherItem{ 
  date: chrono::DateTime<chrono::Local>, 
  location: String, 
  metrics: std::collections::HashMap<MetricType, Box<dyn std::fmt::Display>
} 

fn main() {
  // get values from config file, not sure how. It would be nice if we could feed the config file into clap.rs somehow
  // github issuses mentioning this:
  // https://github.com/clap-rs/clap/issues/1693
  // https://github.com/clap-rs/clap/issues/748
  // https://github.com/clap-rs/clap/issues/251

  // functions returning values should be pure functions, outside of throwing errors
  let _matches = input::get_command_line_input();
  let options = input::parse_matches_into_options(_matches);
  println!("{:?}", options);
  // let request = options.api.build_request(&options);
  // let result = http_request::get_result_from_request(request);
  // cache::cache_result(&result, options.cache_duration);
  // let weather_parsed_result = options.api.parse_result(&result);
  let output = output_generator::generate_output(weather_parsed_result, options, terminal_size::terminal_size().unwrap().0.w);
  output.iter.forEach(|line| println!(line));
}
