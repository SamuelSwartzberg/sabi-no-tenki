use serde_yaml;
use crate::weather_items::{WeatherItem, MetricType};
use crate::prog_options::ProgOptions;

fn reduce_to_significant_figures(weather_items: &Vec<WeatherItem>) -> Vec<WeatherItem>{
  weather_items.iter().map(|item| {
    for key in item.metrics.keys(){
      let val = item.metrics.get(key);
      item.metrics.set(key, (val * 100.0).round() / 100.0);
    }
    item
  }).collect()
}

fn format_weather_type_as_emoji_or_text(weather_items: &Vec<WeatherItem>) -> Vec<WeatherItem> {
  weather_items.iter().map(|item| {
    let weather_type = item.get(MetricType::WeatherType);
    item.set(MetricType::WeatherType, weather_type.get_relevant_message.unwrap())
  }).collect()
}

fn to_yaml_string(weather_items: Vec<WeatherItem>) -> Vec<String>{
  let weather_map_vec = Vec::new();
  for weather_item in weather_items{
    let weather_item_map = std::collections::HashMap<String, serde_yaml::Value>::new();
    weather_item_map.insert("date", serde_yaml::to_value(weather_item.time).unwrap());
    weather_item_map.insert("location", serde_yaml::to_value(weather_item.location).unwrap());
    weather_item_map.insert("metrics", serde_yaml::to_value(weather_item.metrics).unwrap());
    weather_map_vec.push(weather_item_map);
  }
  serde_yaml::to_string(&weather_map_vec).unwrap().lines().map(|item| item.to_string()).collect()
}

fn build_blocks_of_output(weather_items: Vec<WeatherItem>) -> Vec<Vec<String>>{
  let mut output_blocks_vector: Vec<Vec<String>> = Vec::new();
  for weather_item in weather_items{
    let mut output_block: Vec<String> = Vec::new();
    output_block.push(weather_item.time.format("%F %R").to_string());
    weather_item.metrics.into_iter().for_each(|(_, value)| output_block.push(value)) ;// not quite sure what the syntax here is
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
    output_block = output_block.iter().map(|string_item| format!{"{:width$}", string_item, width=longest_line_in_block_length}).collect(); // pad each item in the block
    current_block_line.push(output_block);
  }
  block_line_vector
}

fn put_content_into_lines(block_line_vector: Vec<Vec<Vec<String>>>) -> Vec<String>{
  let mut line_vector: Vec<String> = Vec::new();
  for block_line in block_line_vector{
    let mut block_line_vector: Vec<String> = Vec::new();
    block_line_vector.resize(block_line[0].len(), "".to_string());
    for block in block_line{
      let mut index = 0; //eww
      for line_of_block in block{
        block_line_vector[index] = line_of_block;
      }
    }
    line_vector.append(&mut block_line_vector);
  }
  line_vector
}

pub fn generate_output(weather_items:  &Vec<WeatherItem>, options: &ProgOptions, max_line_length: usize) -> Vec<String>{ 
  let weather_items = reduce_to_significant_figures(&weather_items);
  let weather_items = format_weather_type_as_emoji_or_text(&weather_items);
  if options.human_readable == false{
    to_yaml_string(weather_items)
  } else {
    let location = weather_items[0].location;
    let output_blocks_vector = build_blocks_of_output(weather_items);
    let block_line_vector = get_blocks_for_each_line(max_line_length, output_blocks_vector);
    let mut lines: Vec<String> = Vec::new();
    lines.push(location);
    lines.append(&mut put_content_into_lines(block_line_vector));
    lines
  }
} 
