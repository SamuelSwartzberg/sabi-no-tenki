// use crate::{ProgOptions, WeatherItem, MetricType};
// use std::collections::HashMap;

pub trait Query{
 // fn build_request(&self, options: &ProgOptions );
 // fn parse_result(&self) -> Vec<WeatherItem>;
   fn get_names(&self) -> Vec<&str>;
 // fn transform_generic_metric_name_to_api_specific(&self, metrics: &Vec<MetricType>) -> Vec<String>;
}
pub fn get_api( _api_name: &str) -> Option<Box<dyn Query>>{
    let metaweather_query =  MetaweatherQuery{};
   // let apis: [Box<dyn Query>; 1] = [Box::new(metaweather_query)];
   // let mut target_api: Option<&Box<dyn Query>> = None;
   // let mut api_iter = apis.into_iter();
   // while let Some(api) = api_iter.next().clone(){
   //   if api.get_names().contains(&api_name){target_api = Some(api)};
   // } 
   // return target_api;
   // currently does not work, some problem with ownership no matter what I do
   Some(Box::new(metaweather_query))
}

struct MetaweatherQuery{
//  generic_metric_names_map: HashMap<MetricType, &str>
}
//
impl Query for MetaweatherQuery{
//  fn build_request(options: &ProgOptions ){};
//  fn parse_result() -> Vec<WeatherItem>{};
  fn get_names(&self) -> Vec<&str>{
    vec!["metaweather", "mw"]     
  }
}
//

//   metaweather, //https://www.metaweather.com/api/#locationsearch
//  Weather-related response fields
// weather_state_name
  // Snow
  // Sleet
  // Hail
  // Thunderstorm
  // Heavy Rain
  // Light Rain
  // Showers
  // Heavy Cloud
  // Light Cloud
  // Clear
// weather_state_abbr
// wind_speed	
// wind_direction	
// wind_direction_compass	
// (min|max|the)_temp	
// air_pressure	
// humidity	
// visibility
// predictability	
//   weatherstack, //https://weatherstack.com
// Weather-related response fields
// temperature
// wind_spped
// wind_degree
// wind_dir
// weather_code
// weather_icons
// weather_description
// precipitation
// humidity
// visibility
// pressure
// "cloudcover": 75,
// "heatindex": 18,
// "dewpoint": 12,
// "windchill": 18,
// "windgust": 35,
// "feelslike": 18,
// "chanceofrain": 0,
// "chanceofremdry": 87,
// "chanceofwindy": 0,
// "chanceofovercast": 90,
// "chanceofsunshine": 15,
// "chanceoffrost": 0,
// "chanceofhightemp": 0,
// "chanceoffog": 0,
// "chanceofsnow": 0,
// "chanceofthunder": 0,
// "uv_index": 0

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
