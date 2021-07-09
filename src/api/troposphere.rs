use crate::prog_options::ProgOptions;
use serde_json::{Result, Value};
use crate::weather_items::{WeatherItem, MetricType};


const API_KEY: &str = "9580e6ee934dcdaa209c5ea6b3de9f939f23d0a2e9975a9181";
const BASE_URL: &str = "https://api.troposphere.io/";
const LOCATION_PATH: &str = "place/name/";
const FORECAST_PATH: &str = "forecast/";

fn get_metric_for_local_name(name: &str) -> Option<MetricType>{
  match name{
    "temperature" => TemperatureCur,
    "temperatureMin" => TemperatureMin,
    "temperatureMax" => TemperatureMax,
    "windSpeed" => WindSpeed,
    "relHumidity" => Humidity,
    "preasure"/*sic*/ => Pressure,
    "totalPrecipitation" => Precipitation,
    "uvIndex" => UvIndex,
    "airQualityIndex" => AirQuality
    _ => 
  }
}

pub fn build_location_requests(locations: &Vec<String>) -> Vec<String>{
  let mut requests: Vec<String> = vec![];
  for location in locations{
    requests.push(format!("{}{}{}?token={}", BASE_URL, LOCATION_PATH, location, API_KEY));
  }
  requests
}

pub fn parse_location_results(results: Vec<String>) -> Vec<(f64, f64)>{
  let mut coordinates: Vec<(f64, f64)> = vec![];
  for result in results{
    let result_json: Value = serde_json::from_str(&result).unwrap();
    let first_location = &result_json["data"][0];
    println!("{:?}", first_location);
    coordinates.push((first_location["latitude"].as_f64().unwrap(), first_location["longitude"].as_f64().unwrap()))
  }
  coordinates
}
pub fn build_requests(prog_options: &ProgOptions, locations: Vec<(f64, f64)>) -> Vec<String>{
  let mut requests: Vec<String> = vec![];
  for location in locations{
    requests.push(format!("{}{}{},{}?token={}", BASE_URL, FORECAST_PATH, location.0.to_string(), location.1.to_string(), API_KEY));
  }
  requests
}

fn get_relevant_time_list()

pub fn parse_results(results: Vec<String>) -> Vec<WeatherItems>{
  let mut weather_items: Vec<WeatherItems> = vec![];
  for result in results{
    let result_json: Value = serde_json::from_str(&result).unwrap();

  }
  
}
//   troposphere //https://www.troposphere.io/developer
// response fields
// type	Weather type
  // clear
  // partly-cloudy
  // cloudy
  // dust
  // mist
  // fog 
  // rain
  // snow
  // sandstorm
  // snowdrifting
  // drizzle
  // rain-freezing
  // sleet
  // rain-snow
  // rain-snow-shower
  // snow-shower
  // snow-hail
  // hail
  // thunderstorm
// temperature	Current temperature in °C
// temperatureMin	Minimum temperature in °C
// temperatureMax	Maximum temperature in °C
// windSpeed	Wind speed in m/s
// windGustsSpeed	Wind gusts speed in m/s
// windBearing	Wind bearing in degree
// relHumidity	Relative humidity
// preasure	Preasure in pascal
// totalPrecipitation	Total precipitation in liter/m2
// rain	Rain in liter/m2
// snow	Snow in liter/m2
// uvIndex	UV index
// airQualityIndex	Air quality index
//
