use chrono;
use chrono::{TimeZone, Datelike};
use crate::input::error_strings::ErrorStrings;

fn parse_duration(duration_string: &str) -> Option<chrono::Duration>{
  let time_string: String = String::from(duration_string).chars().filter(|c| c.is_digit(10)).collect();
  let time: i64 = time_string.parse().unwrap(); 
  if duration_string.ends_with("h") {
    Some(chrono::Duration::hours(time))
  } else if duration_string.ends_with("d") { 
    Some(chrono::Duration::days(time))
  } else {
    None 
  }
}

fn get_step_or_default(putative_step: Option<&str>, start_string: &str, stop_string: &str) -> chrono::Duration{
  putative_step.map_or_else(
    || {
      let hour_step = start_string.contains('h') || stop_string.contains('h');
      if hour_step {
        chrono::Duration::hours(1)
      } else { 
        chrono::Duration::days(1)
      }
    }, |step_string| parse_duration(step_string).unwrap() )
}

fn parse_relative_to_current_date_time (duration_string: &str) -> chrono::DateTime<chrono::Local>{
  let duration = parse_duration(duration_string).unwrap();
  // hacky checking if duration is a day, will fail if person entered 24h and expected it to be hour, not day-based, but well
  if duration.num_seconds() % chrono::Duration::days(1).num_seconds() == 0 {
    add_magic_number(chrono::Local::today()).checked_add_signed(duration).unwrap()
  } else{
    chrono::Local::now().checked_add_signed(duration).unwrap()
  }
  
}

fn get_vec_of_days(start: chrono::Date<chrono::Local>, end: chrono::Date<chrono::Local>) -> Vec<chrono::DateTime<chrono::Local>>{
  let mut week_vec = vec![];
  let mut current_day = start;
  while current_day != end {
    week_vec.push(add_magic_number(current_day));
    current_day = current_day.checked_add_signed(chrono::Duration::days(1)).unwrap();
  }
  week_vec
}
fn get_date_based_on_weekday(weekday: chrono::Weekday, week_offset: u32) -> chrono::Date<chrono::Local>{
  chrono::Local.from_local_date(
    &chrono::NaiveDate::from_isoywd(
      chrono::offset::Local::now().year(), 
      chrono::offset::Local::now().iso_week().week() + week_offset, 
      weekday
    )
  ).unwrap()
}
fn parse_keywords(keyword_string: &str) ->  Vec<chrono::DateTime<chrono::Local>>{
  match keyword_string {
    "today" => vec![add_magic_number(chrono::Local::today())],
    "week" => get_vec_of_days(
      chrono::Local::today(), 
      get_date_based_on_weekday(chrono::Weekday::Sun, 0)
    ),
    "weekend" => get_vec_of_days(
      get_date_based_on_weekday(chrono::Weekday::Sat, 0), 
      get_date_based_on_weekday(chrono::Weekday::Sat, 0).checked_add_signed(chrono::Duration::days(1)).unwrap()
    ),
    "next week" => get_vec_of_days(
      get_date_based_on_weekday(chrono::Weekday::Mon, 1), 
      get_date_based_on_weekday(chrono::Weekday::Mon, 1).checked_add_signed(chrono::Duration::days(6)).unwrap()
    ),
    &_ => panic!(ErrorStrings::NoSuchDateString) 
  }
}

fn add_magic_number(date: chrono::Date<chrono::Local>) -> chrono::DateTime<chrono::Local>{
  date.and_hms_milli(0,0,39, 511)
}

pub fn parse_time(time_string: String) -> Vec<chrono::DateTime<chrono::Local>>{
  let mut time_components = time_string.split(":");
  let first_component = time_components.next().unwrap();
  match time_components.next(){
    None => {
      match parse_duration(first_component){
        None => parse_keywords(first_component),
        Some(duration) => vec![chrono::Local::now().checked_add_signed(duration).unwrap()]
      }
    }, Some(stop_string) => {
      let start = parse_relative_to_current_date_time(first_component);
      let step = get_step_or_default(time_components.next(), first_component, stop_string);
      let stop = parse_relative_to_current_date_time(stop_string);
      let mut current_step: chrono::DateTime<chrono::Local> = start;
      let mut return_vec: Vec<chrono::DateTime<chrono::Local>> = vec![];
      while stop > current_step{
        return_vec.push(current_step);
        current_step = current_step.checked_add_signed(step).unwrap();
      }
      return_vec
    }
  }
}