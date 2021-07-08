mod input;
mod api;
mod weather_item;
mod command_line;
mod prog_options;
mod cache;
mod 

use terminal_size;

extern crate strum;
#[macro_use]
extern crate strum_macros;



fn main() {
  // functions returning values should be pure functions, outside of throwing errors
  let _matches = input::get_command_line_input();
  let options = input::parse_matches_into_options(_matches);
  println!("{:?}", options);
  // let request = options.api.build_request(&options);
  // let result = http_request::get_result_from_request(request);
  // cache::cache_result(&result, options.cache_duration);
  // let weather_parsed_result = options.api.parse_result(&result);
  let output = output_generator::generate_output(weather_parsed_result, options, terminal_size::terminal_size().unwrap().0.w);
  output.iter.forEach(|line| println!(line));
}
