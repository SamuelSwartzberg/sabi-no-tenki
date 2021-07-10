mod input;
mod api;
mod weather_items;
mod command_line;
mod get_path;
mod prog_options;
mod http_request;
mod error_strings;

// mod cache;
mod output_generator;

use terminal_size;



fn main() {
  // functions returning values should be pure functions, outside of throwing errors
  let matches = command_line::get_command_line_input();
  let options = input::parse_matches_into_options(matches);
  let location_requests = api::troposphere::build_location_requests(&options.location_list);
  let location_results = http_request::get_results_from_requests(location_requests).unwrap();
  let locations = api::troposphere::parse_location_results(&location_results);
  let names = api::troposphere::parse_location_results_names(&location_results);
  let requests = api::troposphere::build_requests(&options, locations);
  let results = http_request::get_results_from_requests(requests);
  // cache::cache_result(&result, options.cache_duration);
  let weather_parsed_results = api::troposphere::parse_results(results.unwrap(), &options, names );
  println!("{:#?}", weather_parsed_results);
  // for mut weather_parsed_result in weather_parsed_results{
  //   let output = output_generator::generate_output(&mut weather_parsed_result, &options, usize::from(terminal_size::terminal_size().unwrap().0.0));
  //   for line in output{
  //     println!("{}", line);
  //   }
  // }
}
