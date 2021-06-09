use clap::{Arg, App, SubCommand};
use chrono;
use api;

fn get_command_line_input() -> clap::ArgMatches<'static>  { //possibly remove static lifetimes once I become clear what the lifetimes of App are
    return App::new("Sabi no Tenki")
    .version("0.0")
    .author("Sam S. <me@samswartzberg.com>")
    .help("A terminal command line weather client with fine-grained options and a pretentious and possibly incorrect japanese name.")
    .arg(Arg::with_name("location")
        .short("l")
        .long("location")
        .value_name("LOCATION_LIST")
        .help("Use the specified location. Best specified in options file for default location. List is separated by colons (:)")
        .takes_value(true))
    .arg(Arg::with_name("api")
        .long("api")
        .value_name("API")
        .help("Use specified api. Not all requested metrics are available for all apis.")
        .takes_value(true))
    .arg(Arg::with_name("human-readable")
        .short("h")
        .long("human-readable")
        .help("Use human-readable text instead of CSV-like syntax"))
    .arg(Arg::with_name("ascii-image")
        .short("a")
        .long("ascii-image")
        .help("Include an ascii image for every requested unit"))
    .arg(Arg::with_name("graph")
        .short("g")
        .long("graph")
        .help("Show an ascii graph for the requested metrics")
        .value_name("METRICS")
        .takes_value(true))
    .arg(Arg::with_name("cache-duration")
        .long("cache-duration")
        .help("Specify the duration to cache previous results")
        .value_name("DURATION")
        .takes_value(true))
    .arg(Arg::with_name("significant-figures")
        .long("significant-figures")
        .help("give this amount of significant figures (e.g. --significant-figures 1 -> 25.1 C)")
        .value_name("FIGURES")
        .takes_value(true))
    .arg(Arg::with_name("metrics")
        .long("metrics")
        .help("Specify a list of metrics you would like to recieve, as a non-spaced comma-separated list. Not all requested metrics are available for all apis.")
        .value_name("METRIC-LIST")
        .takes_value(true))
    .arg(Arg::with_name("emoji")
        .long("emoji")
        .help("Show things such as current weather as emoji (e.g. '🌧️'). Can be combined with --text if both are desired (e.g. '🌧️ Rainy')."))
    .arg(Arg::with_name("text")
        .long("text")
        .help("Show things such as current weather as text (e.g. 'Rainy'). Can be combined with --emoji if both are desired (e.g. '🌧️ Rainy')."))
    .arg(Arg::with_name("TIME")
        .help("The time span to fetch weather for:
            time = [start[:end][:step]]] | shorthand-values
            start = time-value
            end = time-value
            step = time-value
            time-value = time,unit
            time = 0...23
            unit = 'd'|'h'
            shorthand-values = 'today'|'week'|'weekend'|'next-week'

            no value - now
            1h - in one hour
            5d - in 5 days
            2w - in two weeks
            1h:10h - in the 9 hours starting one hour from now
            1h:10h:3h - in the in the 9 hours starting one hour from now every three hours
            :6d:2d - in the next six days, every second day


            Leaving out step is usually fine - if you specify a h value for any of the values, 
            it will presume hour-based stepping, otherwise it will presume day-based stepping.")
        .index(1))
    .get_matches();
}



enum MetricType{
  weather_type,
  wind_speed,
  wind_direction,
  temperature_cur,
  temperature_min,
  temperature_max,
  humidity,
  pressure,
  precipitation,
  uv_index,
  air_quality,
  visibility,
  predictability,
  feels_like,
  cloud_cover,
  heat_index,
  dewpoint,
  wind_chill,
  wind_gust,
  feels_like,
  chance_of_rain,
  chance_of_remaining_dry,
  chance_of_windy,
  chance_of_overcast,
  chance_of_sunshine,
  chance_of_frost,
  chance_of_high_temp,
  chance_of_fog,
  chance_of_snow,
  chance_of_thunder
}

struct ProgOptions{
  time_list: Vec<chrono::DateTime>,
  location_list: Vec<&str>,
  api: impl api::Query,
  human_readable: bool,
  significant_figures: i8,
  emoji: bool,
  text: bool,
  graph: Vec<MetricType>,
  cache_duration: chrono::Duration,
  metrics: Vec<MetricType>
}

fn parse_matches_into_options() -> ProgOptions{

}

fn get_result_from_cache() -> Result<???,&str>{

}

fn get_result_from_http_request() -> Result<???,&str>{

}

fn get_result_from_request(){
  get_result_from_cache().or_else(get_result_from_http_request())?
}

struct WeatherItem{
  date: chrono::DateTime,
  location: &str,
  metrics: Vec<Metric>
}



struct ShellSpecs{
  
}

fn generate_output(&Vec<WeatherItem> weather_items, &ProgOptions options, &ShellSpecs shell_specs) -> &str{

}

fn get_shell_specs() -> ShellSpecs{

}

fn print_output(&str output){

}

fn main() {
  // get values from config file, not sure how. It would be nice if we could feed the config file into clap.rs somehow
  // github issuses mentioning this:
  // https://github.com/clap-rs/clap/issues/1693
  // https://github.com/clap-rs/clap/issues/748
  // https://github.com/clap-rs/clap/issues/251

  // functions returning values should be pure functions, outside of throwing errors
  let matches = get_command_line_input();
  let options = parse_matches_into_options(matches);
  let request = options.api.build_request(&options);
  let result = get_result_from_request(request);
  cache_result(&result);
  let weather_parsed_result = options.api.parse_result(&result);
  let output = generate_output(&weather_parsed_result, &options, get_shell_specs());
  print_output(output);
}
