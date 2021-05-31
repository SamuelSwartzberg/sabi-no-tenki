use clap::{Arg, App, SubCommand};

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
        .help("Use specified api")
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

fn parse_matches_into_options(){

}

fn build_request(){

}

fn get_result_from_cache(){

}

fn get_result_from_http_request() -> Result<???,&str>{

}

fn get_result_from_request(){
  get_result_from_cache().or_else(get_result_from_http_request())?
}

fn generate_output(){

}

fn get_shell_specs(){

}

fn print_output(){
  
}

fn main() {
  // get values from config file, not sure how. It would be nice if we could feed the config file into clap.rs somehow
  // github issuses mentioning this:
  // https://github.com/clap-rs/clap/issues/1693
  // https://github.com/clap-rs/clap/issues/748
  // https://github.com/clap-rs/clap/issues/251
  let matches = get_command_line_input();
  let options = parse_matches_into_options(matches);
  let request = build_request(&options);
  let result = get_result_from_request(request);
  cache_result(&result);
  let weather_parsed_result = parse_result(&result);
  let output = generate_output(&weather_parsed_result, &options, get_shell_specs());
  print_output(output);
}
