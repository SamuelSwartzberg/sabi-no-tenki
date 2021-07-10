use serde_yaml;
use crate::weather_items::{WeatherItem, WeatherType, MetricType, get_relevant_message};
use crate::prog_options::ProgOptions;
use std::str::FromStr;
use strum::EnumMessage;
//use indexmap::IndexMap;


fn reduce_to_significant_figures(weather_items: &mut Vec<WeatherItem>, significant_figures: u8){
  for weather_item in weather_items{
    for key in weather_item.metrics.clone().keys(){
      let val = weather_item.metrics.get(key).unwrap();
      if let Ok(float_val) = val.parse::<f32>(){
        weather_item.metrics.insert((*key).clone(), 
          ((float_val * 10f32.powi(significant_figures.into())).round() / 10f32.powi(significant_figures.into())).to_string()
        );
      }
    }
  }
}

fn format_weather_type_as_emoji_or_text(weather_items: &mut Vec<WeatherItem>, emoji: bool, text: bool){
  for weather_item in weather_items{
    if let Some(weather_type) = weather_item.metrics.get(&MetricType::WeatherType){
      weather_item.metrics.insert(MetricType::WeatherType, get_relevant_message(WeatherType::from_str(&weather_type).unwrap(), emoji, text).unwrap());
    }
  }
}

fn to_yaml_string(weather_items: &mut Vec<WeatherItem>) -> Vec<String>{
  let mut weather_map_vec = Vec::new();

  for weather_item in weather_items{
    let mut weather_item_map = std::collections::HashMap::<String, serde_yaml::Value>::new();
    let format_string = if weather_item.is_date {"%F"} else {"%F %R"};
    weather_item_map.insert("date".to_string(), serde_yaml::to_value(weather_item.time.format(format_string).to_string()).unwrap());
    weather_item_map.insert("location".to_string(), serde_yaml::to_value(weather_item.location.clone()).unwrap());
    weather_item_map.insert("metrics".to_string(), serde_yaml::to_value(weather_item.metrics.clone()).unwrap());
    weather_map_vec.push(weather_item_map);
  }
  serde_yaml::to_string(&weather_map_vec).unwrap().lines().map(|item| item.to_string()).collect()
}

fn build_blocks_of_output(weather_items: &mut Vec<WeatherItem>,  metrics: &Vec<MetricType>, labeled_columns: bool) -> Vec<Vec<String>>{
  let mut output_blocks_vector: Vec<Vec<String>> = Vec::new();
  if labeled_columns{ 
    let mut first_column: Vec<String> = vec!["Date".to_string()];
    first_column.append(
      &mut weather_items[0].metrics.keys()
      .filter(|key| metrics.contains(&key))
      .map(|key| key.get_message().unwrap().to_string()).collect::<Vec<String>>()
    );
    output_blocks_vector.push(first_column);
  }
  println!("{:?}", output_blocks_vector);
  for weather_item in weather_items{
    let mut output_block: Vec<String> = Vec::new();
    let format_string = if weather_item.is_date {"%b, %d.%m."} else {"%b, %d.%m. %R"};
    output_block.push(weather_item.time.format(format_string).to_string());
    weather_item.metrics.clone().into_iter().filter(|(key, _)| metrics.contains(&key)).for_each(|(_, value)| output_block.push(value)) ;// not quite sure what the syntax here is
    output_blocks_vector.push(output_block);
  }
  output_blocks_vector
}

fn get_blocks_for_each_line(max_line_length: usize, output_blocks_vector: Vec<Vec<String>>) -> Vec<Vec<Vec<String>>>{
  let mut block_line_vector: Vec<Vec<Vec<String>>> = Vec::new();
  let mut line_length_current: usize = 0;
  let mut current_block_line: Vec<Vec<String>> = Vec::new();
  for output_block in output_blocks_vector{
    let longest_line_in_block_length = output_block.iter().map(|string_item| string_item.len()).max().unwrap();
    if line_length_current + longest_line_in_block_length > max_line_length {
      block_line_vector.push(current_block_line);
      current_block_line = Vec::new();
      line_length_current = 0;
    }
    line_length_current += longest_line_in_block_length;
    let output_block_padded = output_block.iter().map(|string_item| format!{"{:width$}", string_item, width=longest_line_in_block_length+2}).collect(); // pad each item in the block
    current_block_line.push(output_block_padded);
  }
  block_line_vector.push(current_block_line); // last one will not have overflowed
  println!("{:?}", block_line_vector);
  block_line_vector
}

fn put_content_into_lines(block_line_vector: Vec<Vec<Vec<String>>>) -> Vec<String>{
  println!("{:?}", block_line_vector);
  let mut line_vector: Vec<String> = Vec::new();
  for block_line in block_line_vector{
    
    let mut block_line_vector: Vec<String> = Vec::new();
    block_line_vector.resize(block_line[0].len(), "".to_string());
    for block in block_line{
      let mut index = 0; //eww
      println!("{:?}", block);
      for line_of_block in block{
        block_line_vector[index] += &line_of_block;
        index+=1;
      }
      println!("{:?}", block_line_vector);
    }
    line_vector.append(&mut block_line_vector);
  }
  line_vector
}

pub fn generate_output(mut weather_items:  &mut Vec<WeatherItem>, options: &ProgOptions, max_line_length: usize) -> Vec<String>{ 
  reduce_to_significant_figures(&mut weather_items, options.significant_figures);
  format_weather_type_as_emoji_or_text(&mut weather_items, options.emoji, options.text);
  if options.human_readable == false{
    to_yaml_string(weather_items)
  } else {
    let location = weather_items[0].location.clone();
    let output_blocks_vector = build_blocks_of_output(weather_items, &options.metrics, options.labeled_columns);
    println!("{:?}", output_blocks_vector);
    let block_line_vector = get_blocks_for_each_line(max_line_length, output_blocks_vector);
    let mut lines: Vec<String> = Vec::new();
    lines.push(location);
    lines.append(&mut put_content_into_lines(block_line_vector));
    lines
  }
} 
