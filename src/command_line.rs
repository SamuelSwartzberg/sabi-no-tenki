use clap::{Arg, App /*, SubCommand*/};
pub fn get_command_line_input() -> clap::ArgMatches { //possibly remove static lifetimes once I become clear what the lifetimes of App are
    return App::new("Sabi no Tenki")
    .version("0.0")
    .author("Sam S. <me@samswartzberg.com>")
    .about("A terminal command line weather client with fine-grained options and a pretentious and possibly incorrect japanese name. Can also be configured via $XDG_CONFIG_HOME/tenki_config.yaml (.config/tenki_config.yaml). Keys are the same names as options, but using underscores instead. Boolean flags cannot be set via config, as it would not be able to override them from the command line.")
    .arg(Arg::new("location_list")
        .short('l')
        .long("location")
        .value_name("LOCATION_LIST")
        .about("Use the specified location. Best specified in options file for default location. List is separated by colons (:)")
        .takes_value(true))
    .arg(Arg::new("api")
        .long("api")
        .value_name("API")
        .about("Use specified api. Not all requested metrics are available for all apis. Currently unimplemented.")
        .takes_value(true))
    .arg(Arg::new("human_readable")
        .short('h')
        .long("human-readable")
        .about("Use human-readable text instead of CSV-like syntax"))
    .arg(Arg::new("ascii_image")
        .short('a')
        .long("ascii-image")
        .about("Include an ascii image for every requested unit"))
    .arg(Arg::new("graph")
        .short('g')
        .long("graph")
        .about("Show an ascii graph for the requested metrics. Currently unimplemented.")
        .value_name("METRICS")
        .takes_value(true))
    .arg(Arg::new("cache_duration")
        .long("cache-duration")
        .about("Specify the age of cached results you accept. Currently only caches the previous request.")
        .value_name("DURATION")
        .takes_value(true))
    .arg(Arg::new("significant_figures")
        .long("significant-figures")
        .about("give this amount of significant figures (e.g. --significant-figures 1 -> 25.1 C)")
        .value_name("FIGURES")
        .takes_value(true))
    .arg(Arg::new("metrics")
        .long("metrics")
        .about("Specify a list of metrics you would like to recieve, as a non-spaced comma-separated list. Not all requested metrics are available for all apis.")
        .value_name("METRIC-LIST")
        .takes_value(true))
    .arg(Arg::new("emoji")
        .long("emoji")
        .about("Show things such as current weather as emoji (e.g. '🌧️'). Can be combined with --text if both are desired (e.g. '🌧️ Rainy')."))
    .arg(Arg::new("text")
        .long("text")
        .about("Show things such as current weather as text (e.g. 'Rainy'). Can be combined with --emoji if both are desired (e.g. '🌧️ Rainy')."))
    .arg(Arg::new("week_starts_sat")
        .long("week-sat")
        .about("Week starts Saturday"))
    .arg(Arg::new("labeled_columns")
        .long("labeled-columns")
        .about("Show the names of the metrics at the beginning of the first column."))
    .arg(Arg::new("week_starts_sun")
        .long("week-sun")
        .about("Week starts Sunday"))
    .arg(Arg::new("time_list")
        .about("The time span to fetch weather for:
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
