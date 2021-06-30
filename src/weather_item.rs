use strum;
use strum_macros;
use chrono;

pub struct WeatherItem{ 
  date: chrono::DateTime<chrono::Local>, 
  location: String, 
  metrics: std::collections::HashMap<MetricType, Box<dyn std::fmt::Display>>
} 
    
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

#[derive(EnumMessage)]
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

impl<T: EnumMessage> WeatherType<T>{
  get_relevant_message(&self, detailed: bool) -> Option<&'static str>{
    match detailed{
      true => self.get_detailed_message(),
      false => self.get_message()
    }
  }
}
