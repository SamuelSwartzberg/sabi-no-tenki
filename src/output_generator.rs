fn reduce_to_significant_figures(weather_items: Vec<WeatherItem>) -> Vec<WeatherItem>{

  weather_items = weather_items.into_iter.map(|item| {
    for key in item.metrics.keys(){
      let val = item.metrics.get(key);
      // get items down to significant figures
    }
    item
  })
}

fn to_yaml_string(weather_items: Vec<WeatherItem>) -> Vec<WeatherItem{
  let weather_map_vec = Vec::new();
  for weather_item in weather_items{
    let weather_item_map = std::collections::HashMap::new();
    weather_item_map.insert("date", weather_item.date);
    weather_item_map.insert("location", weather_item.location);
    weather_item_map.insert("metrics", weather_item.metrics);
    weather_map_vec.push(weather_item_map);
  }
  serde_yaml::to_string(weather_map_vec);
}

fn build_blocks_of_output(&weather_items: Vec<WeatherItem>) -> Vec<Vec<String>>{
  let mut output_blocks_vector: Vec<Vec<String>> = Vec::new();
  for weather_item in weather_items{
    let mut output_block: Vec<String> = Vec::new();
    output_block.push(weather_item.date.format("%F %R"));
    output_block.push(location):
    weather_item.metrics.iter().for_each(|_, value| output_block.push(value)) ;// not quite sure what the syntax here is
    output_blocks_vector.push(output_block);
  }
  output_blocks_vector
}

fn get_blocks_for_each_line(max_line_length: u16, &output_blocks_vector: Vec<Vec<String>>) -> Vec<Vec<String>>{
  let mut line_vector: Vec<Vec<String>> = Vec::new();
  let mut line_length_current: u16 = 0;
  let mut current_line: Vec<String> = Vec::new();
  for output_block in output_blocks_vector{
    let block_max_size = output_block.iter().map(|string_item| string_item.len()).iter().max();
    if line_length_current + block_max_size > max_line_length {
      line_vector.push(current_line);
      current_line = Vec::new();
      line_length_current = 0;
    }
    line_length_current += block_max_size;
    output_block = output_block.iter().map(|string_item| format!{"{:width$}", string_item, width=block_max_size}); // pad each item in the block
    current_line.push(output_block);
  }
  line_vector
}

fn put_content_into_lines(line_vector: Vec<Vec<String>>) -> Vec<String>{
  line_vector.into_iter().map(|line| line.into_iter().reduce(|acc, item| acc + &item).unwrap())
}

fn generate_output(&weather_items: Vec<WeatherItem>, &options: ProgOptions, max_line_length: u16) -> Vec<String>{ 
  weather_items = reduce_to_significant_figures(weather_items);
  weather_items = format_weather_type_as_emoji_or_text(weather_items);
  if options.human_readable = false{
    to_yaml_string(weather_items)
  } else {
    let output_blocks_vector = build_block_of_output(weather_items);
    let line_vector = get_blocks_for_each_line(max_line_length, output_blocks_vector);
    put_content_into_lines(line_vector)
  }
} 
