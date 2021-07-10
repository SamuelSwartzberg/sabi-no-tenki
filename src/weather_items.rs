use strum;
use strum::EnumMessage;
use strum_macros::EnumString;
use chrono;
use serde::{Serialize};

#[derive(Debug)]
pub struct WeatherItem{ 
  pub time: chrono::DateTime<chrono::FixedOffset>, 
  pub location: String, 
  pub metrics: std::collections::HashMap<MetricType, String>
} 
    
#[derive(EnumString, Debug, PartialEq, Eq, Hash, Serialize, Clone )] 
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