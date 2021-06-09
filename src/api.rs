
trait Query{
  fn build_request(&ProgOptions options);
  fn parse_result() -> Vec<WeatherItem>;
  fn is_name_of(&str) -> bool;
  fn transform_generic_metric_name_to_api_specific(&Vec<MetricType>) -> Vec<String>;
}
fn get_api(&str api_name) -> impl Query{

}

struct MetaweatherQuery{
  names: Vec<&str>,
  generic_metric_names_map: HashMap
}

impl Query for MetaweatherQuery{
  fn build_request(&ProgOptions options);
  fn parse_result() -> Vec<WeatherItem>;
  fn is_name_of(&str name) -> bool{
    names.contains(name);
  }
}

let metaweather_query = struct MetaweatherQuery{
  names: ["metaweather", "mw"]
}

let apis[1, impl Query] = [metaweather_query]



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
