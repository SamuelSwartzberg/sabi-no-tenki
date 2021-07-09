use crate::prog_options::ProgOptions;
use chrono::{Local, Timelike};
use serde_json::{Result, Value};
use crate::weather_items::{WeatherItem, MetricType};


const API_KEY: &str = "9580e6ee934dcdaa209c5ea6b3de9f939f23d0a2e9975a9181";
const BASE_URL: &str = "https://api.troposphere.io/";
const LOCATION_PATH: &str = "place/name/";
const FORECAST_PATH: &str = "forecast/";

fn get_metric_for_local_name(name: &str) -> Option<MetricType>{
  match name{
    "temperature" => Some(MetricType::TemperatureCur),
    "temperatureMin" => Some(MetricType::TemperatureMin),
    "temperatureMax" => Some(MetricType::TemperatureMax),
    "windSpeed" => Some(MetricType::WindSpeed),
    "relHumidity" => Some(MetricType::Humidity),
    "preasure"/*sic*/ => Some(MetricType::Pressure),
    "totalPrecipitation" => Some(MetricType::Precipitation),
    "uvIndex" => Some(MetricType::UvIndex),
    "airQualityIndex" => Some(MetricType::AirQuality),
    _ => None
  }
}

pub fn build_location_requests(locations: &Vec<String>) -> Vec<String>{
  let mut requests: Vec<String> = vec![];
  for location in locations{
    requests.push(format!("{}{}{}?token={}", BASE_URL, LOCATION_PATH, location, API_KEY));
  }
  requests
}

pub fn parse_location_results(results: &Vec<String>) -> Vec<(f64, f64)>{
  let mut coordinates: Vec<(f64, f64)> = vec![];
  for result in results{
    let result_json: Value = serde_json::from_str(&result).unwrap();
    let first_location = &result_json["data"][0];
    println!("{:?}", first_location);
    coordinates.push((first_location["latitude"].as_f64().unwrap(), first_location["longitude"].as_f64().unwrap()))
  }
  coordinates
}

pub fn parse_location_results_names(results: &Vec<String>) -> Vec<String>{
  let mut names: Vec<String> = vec![];
  for result in results{
    let result_json: Value = serde_json::from_str(&result).unwrap();
    let first_location = &result_json["data"][0];
    names.push(first_location["name"].as_str().unwrap() + ", " + &first_location["country"].as_str().unwrap())
  }
  names
}

pub fn build_requests(prog_options: &ProgOptions, locations: Vec<(f64, f64)>) -> Vec<String>{
  let mut requests: Vec<String> = vec![];
  for location in locations{
    requests.push(format!("{}{}{},{}?token={}", BASE_URL, FORECAST_PATH, location.0.to_string(), location.1.to_string(), API_KEY));
  }
  requests
}

fn time_to_nearest_hour(time: chrono::DateTime<Local>) -> chrono::DateTime<Local>{
  time.date().and_hms(time.hour(),0,0)
}

fn get_relevant_time_list(time_list: Vec<chrono::DateTime<Local>>) -> Vec<chrono::DateTime<Local>>{
  let mut new_time_list = time_list.into_iter().map(time_to_nearest_hour).collect::Vec<chrono::DateTime<Local>>()
  new_time_list.dedup();
  new_time_list
}

fn get_relevant_date_list(time_list: Vec<chrono::DateTime<Local>>) -> Vec<chrono::Date<Local>>{
  time_list.into_iter().map(|time| time.date()).collect().dedup()
}

pub fn parse_results(results: Vec<String>, prog_options: &ProgOptions, location_names: Vec<String>) -> Vec<Vec<WeatherItem>>{
  let mut weather_items_different_locations: Vec<WeatherItem> = vec![];
  for (result, location) in results.iter().zip(location_names.iter()){
    let mut weather_items: Vec<WeatherItem> = vec![];
    let result_json: Value = serde_json::from_str(&result).unwrap();
    let results = result_json["data"];
    if prog_options.time_list[0].nanosecond() == 414269896{
      let results_time_array = results["daily"];
      let relevant_times = get_relevant_date_list(prog_options.time_list);
    } else {
      let results_time_array = results["hourly"];
      let relevant_times = get_relevant_time_list(prog_options.time_list);
    }
    for result_time in results_time_array{
      if let Some(time_mapping) = result_time.as_object(){
        if let Ok(time) = chrono::DateTime::parse_from_rfc3339(time_mapping.get("time")){
          if relevant_times.contains(time) {
            let mut metrics = std::collections::HashMap<MetricType, String>::new();
            for (key, value) in time_mapping{
              if let Some(key_enum_val) = get_metric_for_local_name(key){
                metrics.insert(key_enum_val, value);
              }
            }
            weather_items.push(WeatherItem{
              time: time,
              location: location,
              metrics: metrics
            }):
          }
        }
      }
    }
    weather_items_different_locations.push(weather_items);
  }
  weather_items_different_locations
  
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
