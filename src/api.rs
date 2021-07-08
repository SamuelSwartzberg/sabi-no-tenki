//! Adding apis is done by creating the relevant structs, methods etc. in their own source file in
//! `api/`, and then adding the api to the apis array here.

// use crate::{ProgOptions, WeatherItem, MetricType};
// use std::collections::HashMap;
pub mod metaweather_query;

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



