use crate::prog_options::ProgOptions;
use chrono::{Local, FixedOffset, Timelike};
use serde_json::{/*Result,*/Value};
use crate::weather_items::{WeatherItem, MetricType, WeatherType};
use crate::error_strings::{ErrorStrings, err_str};
use indexmap::IndexMap;


const API_KEY: &str = "9580e6ee934dcdaa209c5ea6b3de9f939f23d0a2e9975a9181";
const BASE_URL: &str = "https://api.troposphere.io/";
const LOCATION_PATH: &str = "place/name/";
const FORECAST_PATH: &str = "forecast/";

fn get_metric_for_local_name(name: &str) -> Option<MetricType>{
  match name{
    "temperature" => Some(MetricType::Temperature),
    "temperatureMin" => Some(MetricType::TemperatureMin),
    "temperatureMax" => Some(MetricType::TemperatureMax),
    "windSpeed" => Some(MetricType::WindSpeed),
    "relHumidity" => Some(MetricType::Humidity),
    "preasure"/*sic*/ => Some(MetricType::Pressure),
    "totalPrecipitation" => Some(MetricType::Precipitation),
    "uvIndex" => Some(MetricType::UvIndex),
    "airQualityIndex" => Some(MetricType::AirQuality),
    "type" => Some(MetricType::WeatherType),
    _ => None
  }
}
fn get_weather_type_for_local_name(name: &str) -> Option<WeatherType>{
  match name{  
    "clear" => Some(WeatherType::Clear),
    "partly-cloudy" => Some(WeatherType::PartlyCloudy),
    "cloudy" => Some(WeatherType::Cloudy),
    "dust" => Some(WeatherType::Dust),
    "mist" => Some(WeatherType::Mist),
    "fog" => Some(WeatherType::Fog),
    "rain" => Some(WeatherType::Rain),
    "snow" => Some(WeatherType::Snow),
    "sandstorm" => Some(WeatherType::Sandstorm),
    "snow-shower" => Some(WeatherType::SnowShower),
    "rain-snow-showers" => Some(WeatherType::RainSnowShower),
    "drizzle" => Some(WeatherType::LightRain), //also drizzle    
    "rain-showers" => Some(WeatherType::RainShower),
    "sleet" => Some(WeatherType::Sleet),
    "rain-freezing" => Some(WeatherType::FreezingRain),
    "hail" => Some(WeatherType::Hail),
    "thunderstorm" => Some(WeatherType::Thunderstorms),
    "snow-hail" => Some(WeatherType::Snow),
    "snowdrifting" => Some(WeatherType::Snow),
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

pub fn parse_location_results(results: &Vec<Value>) -> Vec<(f64, f64)>{
  let mut coordinates: Vec<(f64, f64)> = vec![];
  for result in results{
    let first_location = &result["data"][0];
    coordinates.push(
      (
        first_location["latitude"].as_f64()
        .expect(&("latitude".to_owned() + err_str(ErrorStrings::NotInExpectedPlace))), 
        first_location["longitude"].as_f64()
        .expect(&("Longitude".to_owned() + err_str(ErrorStrings::NotInExpectedPlace)))
      )
    )
  }
  coordinates
}

pub fn parse_location_results_names(results: &Vec<Value>) -> Vec<String>{
  let mut names: Vec<String> = vec![];
  for result in results{
    let first_location = &result["data"][0];
    names.push(first_location["name"].as_str().expect(&("Name".to_owned() + err_str(ErrorStrings::NotInExpectedPlace))).to_string() + ", " + &first_location["country"].as_str().expect(&("Country".to_owned() + err_str(ErrorStrings::NotInExpectedPlace))))
  }
  names
}

/// prog_options is not used here, but might for other apis, so it's here for consistency
pub fn build_requests(_prog_options: &ProgOptions, locations: Vec<(f64, f64)>) -> Vec<String>{
  let mut requests: Vec<String> = vec![];
  for location in locations{
    requests.push(format!("{}{}{},{}?token={}", BASE_URL, FORECAST_PATH, location.0.to_string(), location.1.to_string(), API_KEY));
  }
  requests
}

fn time_to_nearest_hour(time: chrono::DateTime<Local>) -> chrono::DateTime<Local>{
  time.date().and_hms(time.hour(),0,0)
}

fn get_relevant_time_list(time_list: Vec<chrono::DateTime<Local>>) -> Vec<chrono::DateTime<FixedOffset>>{
  let mut new_time_list = time_list.into_iter().map(time_to_nearest_hour).map(local_to_fixed).collect::<Vec<chrono::DateTime<FixedOffset>>>();
  new_time_list.dedup();
  new_time_list
}

fn get_relevant_date_list(time_list: Vec<chrono::DateTime<Local>>) -> Vec<chrono::Date<FixedOffset>>{
  let mut new_time_list = time_list.into_iter().map(local_to_fixed).map(|time| time.date()).collect::<Vec<chrono::Date<FixedOffset>>>();
  new_time_list.dedup();
  new_time_list
}

fn assemble_weather_item(time_mapping: &serde_json::Map<String, Value>, time: chrono::DateTime<FixedOffset>, location: String, is_date: bool) -> WeatherItem{
  let mut metrics: IndexMap<MetricType, String> = IndexMap::new();
  for (key, value) in time_mapping{
    if let Some(key_enum_val) = get_metric_for_local_name(key){
      let mut value_as_string: String = if value.is_string(){
        value.as_str().unwrap().to_string() // safe unwrap since we've checked
      } else {
        value.to_string()
      };
      if key_enum_val == MetricType::WeatherType{
        value_as_string = get_weather_type_for_local_name(&value_as_string).expect(err_str(ErrorStrings::NoSuchWeatherType)).to_string();
      } 
      metrics.insert(key_enum_val, value_as_string);
    }
  }
  
  WeatherItem{
    time: time,
    location: location.clone(),
    metrics: metrics,
    is_date: is_date
  }
}

fn insert_into_weather_items_if_valid_unique_time(weather_items: &mut Vec<WeatherItem>, result_time: serde_json::Value, location: String, is_relevant_time: &dyn Fn(chrono::DateTime<FixedOffset>) -> bool, is_date: bool){
  if let Some(time_mapping) = result_time.as_object(){
    if let Some(time_value) = time_mapping.get("time"){
      if let Ok(time) = time_value.as_str().expect(&("Time value".to_owned() + err_str(ErrorStrings::NotOfExpectedType))).parse::<chrono::DateTime<FixedOffset>>(){
        if is_relevant_time(time) {
          weather_items.push(assemble_weather_item(time_mapping, time, location, is_date));
        }
      }
    }
  }
}

fn is_date(first_time: &chrono::DateTime<Local>)-> bool{
  first_time.nanosecond() == 414269896
}

fn get_relevant_results_time_array(results: &serde_json::Map<String, Value>, first_time: &chrono::DateTime<Local>) -> Vec<serde_json::Value> {
  results.get(if is_date(&first_time) {"daily"} else {"hourly"}).expect(&("Array of time values".to_owned() + err_str(ErrorStrings::NotInExpectedPlace))).as_array().expect(&("Array of time values".to_owned() + err_str(ErrorStrings::NotOfExpectedType))).clone()
}

fn local_to_fixed(local_date_time: chrono::DateTime<Local>) -> chrono::DateTime<FixedOffset> {
  local_date_time.with_timezone(local_date_time.offset())
}


pub fn parse_results(results: Vec<Value>, prog_options: &ProgOptions, location_names: Vec<String>) -> Vec<Vec<WeatherItem>>{
  
  let mut weather_items_different_locations: Vec<Vec<WeatherItem>> = vec![];
  
  for (result, location) in results.iter().zip(location_names.into_iter()){

    let mut weather_items: Vec<WeatherItem> = vec![];
    let result_data = result["data"].as_object().expect(&("Data of API results".to_owned() + err_str(ErrorStrings::NotOfExpectedType)));

    for result_time in get_relevant_results_time_array(result_data, &prog_options.time_list[0]){
      if is_date(&prog_options.time_list.clone()[0]){
        insert_into_weather_items_if_valid_unique_time(
          &mut weather_items, 
          result_time, 
          location.clone(),
          &|time| get_relevant_date_list(prog_options.time_list.clone()).contains(&time.date()),
          true
        );
      } else {
        insert_into_weather_items_if_valid_unique_time(
          &mut weather_items, 
          result_time, 
          location.clone(),
          &|time| get_relevant_time_list(prog_options.time_list.clone()).contains(&time),
          false
        );
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
