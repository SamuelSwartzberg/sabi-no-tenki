use strum;
use strum::EnumMessage;
use strum_macros::EnumString;
use chrono;
use serde::{Serialize};
use indexmap::IndexMap;


#[derive(Debug)]
pub struct WeatherItem{ 
  pub time: chrono::DateTime<chrono::FixedOffset>, 
  pub location: String, 
  pub metrics: IndexMap<MetricType, String>
} 
    
#[derive(strum_macros::EnumMessage, EnumString, Debug,  strum_macros::ToString, PartialEq, Eq, Hash, Serialize, Clone )] 
#[strum(ascii_case_insensitive)]
pub enum MetricType{
  #[strum(message="Weather Type")]
  WeatherType,
  #[strum(message="Wind Speed (m/s)")]
  WindSpeed,
  #[strum(message="Wind direction (°)")]
  WindDirection,
  #[strum(message="Temperature (°C)")]
  Temperature,
  #[strum(message="Maximum Temperature (°C)")]
  TemperatureMin,
  #[strum(message="Minimum Temperature (°C)")]
  TemperatureMax,
  #[strum(message="Humidity (%)")]
  Humidity,
  #[strum(message="Pressure (Pa)")]
  Pressure,
  #[strum(message="Precipitation (l/m2)")]
  Precipitation,
  #[strum(message="UV index (1-10)")]
  UvIndex,
  #[strum(message="Air Quality (diff.)")]
  AirQuality,
  #[strum(message="Visibility")]
  Visibility,
  #[strum(message="Weather Type")]
  Predictability,
  #[strum(message="Cloud Cover")]
  CloudCover,
  #[strum(message="Heat Index")]
  HeatIndex,
  #[strum(message="Dewpoint")]
  Dewpoint,
  #[strum(message="Windchill")]
  Windchill,
  #[strum(message="Wind Gust")]
  WindGust,
  #[strum(message="Feels Like")]
  FeelsLike,
  #[strum(message="Chance of Ran")]
  ChanceOfRain,
  #[strum(message="Chance of Remaining Dry")]
  ChanceOfRemainingDry,
  #[strum(message="Chance of Wind")]
  ChanceOfWindy,
  #[strum(message="Chance of being Overcast")]
  ChanceOfOvercast,
  #[strum(message="Chance of Sunshine")]
  ChanceOfSunshine,
  #[strum(message="Chance of Frost")]
  ChanceOfFrost,
  #[strum(message="Chance of High Temp")]
  ChanceOfHighTemp,
  #[strum(message="Chance of Fog")]
  ChanceOfFog,
  #[strum(message="Chance of Snow")]
  ChanceOfSnow,
  #[strum(message="Chance of Thunder")]
  ChanceOfThunder
}
#[derive(strum_macros::EnumMessage, strum_macros::ToString, EnumString, Debug)]
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
  #[strum(message="🌧️", detailed_message="rain")]    
  Rain,    
  #[strum(message="🌨️", detailed_message="snow")]    
  Snow,    
  #[strum(message="🌨️🏃", detailed_message="snow showers")]      
  SnowShower,    
  #[strum(message="", detailed_message="darude - sandstorm")]    
  Sandstorm,                                                                          
  #[strum(message="🌧️🌨️", detailed_message="rain and snow")]     
  RainSnow,
  #[strum(message="🌧️🌨️🏃", detailed_message="rain and snow showers")]                                                         
  RainSnowShower,    
  #[strum(message="💦", detailed_message="drizzle")]                                                                       
  LightRain, //also drizzle    
  #[strum(message="🌧️🏃", detailed_message="rain showers")]  
  RainShower,    
  #[strum(message="💧❄️", detailed_message="sleet")]    
  Sleet,  
  #[strum(message="❄️🌧️", detailed_message="freezing rain")]   
  FreezingRain,    
  #[strum(message="🤕❄️", detailed_message="hail")]    
  Hail,    
  #[strum(message="🌩️", detailed_message="thunderstorm")]                                                                    
  Thunderstorms     
} 

pub fn get_relevant_message(weather_type: WeatherType, emoji: bool, text: bool) -> Option<String>{
  let mut message = "".to_string();
  if emoji {message = weather_type.get_message()?.to_owned() + " "};
  if text {message += weather_type.get_detailed_message()?};
  Some(message)
}