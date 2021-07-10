mod input;
mod api;
mod weather_items;
mod command_line;
mod get_path;
mod prog_options;
mod http_request;
mod error_strings;
mod cache;
mod output_generator;
mod utils;

use terminal_size;

fn get_results_from_cache_or_http(requests: Vec<String>, cache_type: String) -> Vec<String>{
  if let Some(cache_str) = cache::get_result_from_cache(&options.cache_duration, cache_type, &requests) {
    cache_str
  } else {
    let results = http_request::get_results_from_requests(requests).unwrap();
    cache::cache_result(&results, cache_type);
    results
  }
} 

fn main() {
  // functions returning values should be pure functions, outside of throwing errors
  let matches = command_line::get_command_line_input();
  let options = input::parse_matches_into_options(matches);
  let location_requests = api::troposphere::build_location_requests(&options.location_list);

  let location_results = get_results_from_cache_or_http(location_requests, "location");

  let locations = api::troposphere::parse_location_results(&location_results);
  let names = api::troposphere::parse_location_results_names(&location_results);
  let requests = api::troposphere::build_requests(&options, locations);

  let results = get_results_from_cache_or_http(requests, "weather");

  let weather_parsed_results = api::troposphere::parse_results(results, &options, names );
  println!("{:#?}", weather_parsed_results);
  for mut weather_parsed_result in weather_parsed_results{
    let output = output_generator::generate_output(&mut weather_parsed_result, &options, usize::from(terminal_size::terminal_size().unwrap().0.0));
    for line in output{
      println!("{}", line);
    }
  }
}
