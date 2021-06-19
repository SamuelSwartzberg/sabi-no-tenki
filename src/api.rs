use crate::{ProgOptions, WeatherItem, MetricType};
// use std::collections::HashMap;

pub trait Query{
  fn build_request(&self, options: &ProgOptions );
  fn parse_result(&self) -> Vec<WeatherItem>;
  fn is_name_of(&self, name: &str) -> bool;
  fn transform_generic_metric_name_to_api_specific(&self, metrics: &Vec<MetricType>) -> Vec<String>;
}
pub fn get_api( api_name: &str) -> Option<Box<dyn Query>>{
    let metaweather_query =  MetaweatherQuery{
        names: ["metaweather", "mw"]
    };
    let apis: [Box<dyn Query>] = [metaweather_query];
    let target_api: Option<Box<dyn Query>> = None;
    for api in &apis{
      if api.names.contains(api_name){target_api = api};
    }
    return Some(target_api);

}
//
struct MetaweatherQuery{
  names: [&str],
//  generic_metric_names_map: HashMap<MetricType, &str>
}
//
//impl Query for MetaweatherQuery{
//  fn build_request(options: &ProgOptions ){};
//  fn parse_result() -> Vec<WeatherItem>{};
//  fn is_name_of( name: &str) -> bool{
//    names.contains(name);
//  }
//}
//

//   metaweather, //https://www.metaweather.com/api/#locationsearch
//  Weather-related response fields
// weather_state_name
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
