use chrono;
use chrono::{TimeZone, Datelike};
use crate::error_strings::{ErrorStrings, err_str};
use crate::prog_options::WeekStarts;

fn parse_duration(duration_string: &str) -> Option<chrono::Duration>{
  let time_string: String = String::from(duration_string).chars().filter(|c| c.is_digit(10)).collect();
  match time_string.parse::<i64>().ok() {
    Some(time) => {
      if duration_string.ends_with("h") {
        Some(chrono::Duration::hours(time))
      } else if duration_string.ends_with("d") { 
        Some(chrono::Duration::days(time))
      } else {
        None 
      }
    }
    None =>  None
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
    }, |step_string| parse_duration(step_string).expect(err_str(ErrorStrings::CouldNotParseTimeList)) )
}

fn parse_relative_to_current_date_time (duration_string: &str) -> chrono::DateTime<chrono::Local>{
  let duration = parse_duration(duration_string).expect(err_str(ErrorStrings::CouldNotParseTimeList));
  // hacky checking if duration is a day, will fail if person entered 24h and expected it to be hour, not day-based, but well
  if duration.num_seconds() % chrono::Duration::days(1).num_seconds() == 0 {
    add_magic_number(chrono::Local::today()).checked_add_signed(duration).expect(err_str(ErrorStrings::IntervalTooLarge))
  } else{
    chrono::Local::now().checked_add_signed(duration).expect(err_str(ErrorStrings::IntervalTooLarge))
  }
  
}

fn get_vec_of_days(start: chrono::Date<chrono::Local>, end: chrono::Date<chrono::Local>) -> Vec<chrono::DateTime<chrono::Local>>{
  let mut week_vec = vec![];
  let mut current_day = start;
  if start.and_hms(0,0,0).timestamp() > end.and_hms(0,0,0).timestamp() {panic!("Start in range of time cannot be after end, will cause infinite loop.")};
  while current_day != end {
    week_vec.push(add_magic_number(current_day));
    current_day = current_day.checked_add_signed(chrono::Duration::days(1)).expect(err_str(ErrorStrings::IntervalTooLarge));
  }
  week_vec
}
fn get_date_based_on_weekday(weekday: chrono::Weekday, week_offset: u32) -> chrono::Date<chrono::Local>{
  chrono::Local.from_local_date(
    &chrono::NaiveDate::from_isoywd(
      chrono::offset::Local::now().year(), 
      chrono::offset::Local::now().iso_week().week(), 
      weekday
    )
  ).unwrap().checked_add_signed(chrono::Duration::weeks(week_offset.into())).expect(err_str(ErrorStrings::IntervalTooLarge))
}

fn get_weekday_of_week_end(week_starts: &WeekStarts) -> chrono::Weekday{
  match week_starts{
    WeekStarts::Mon => chrono::Weekday::Sun,
    WeekStarts::Sun => chrono::Weekday::Sat,
    WeekStarts::Sat => chrono::Weekday::Fri,
  }
}

fn get_weekday_of_week_start(week_starts: &WeekStarts) -> chrono::Weekday{
  match week_starts{
    WeekStarts::Mon => chrono::Weekday::Mon,
    WeekStarts::Sun => chrono::Weekday::Sun,
    WeekStarts::Sat => chrono::Weekday::Sat,
  }
}

/// this holds true in most jurisdictions, but not all. 
fn get_weekday_of_weekend_start(week_starts: &WeekStarts) -> chrono::Weekday{ 
  match week_starts{
    WeekStarts::Mon => chrono::Weekday::Sat,
    WeekStarts::Sun => chrono::Weekday::Sat,
    WeekStarts::Sat => chrono::Weekday::Fri,
  }
}
fn parse_keywords(keyword_string: &str, week_starts: &WeekStarts) ->  Vec<chrono::DateTime<chrono::Local>>{
  match keyword_string {
    "today" => vec![add_magic_number(chrono::Local::today())],
    "week" | "this week" => get_vec_of_days(
      chrono::Local::today(), 
      get_date_based_on_weekday(get_weekday_of_week_end(week_starts), 0).checked_add_signed(chrono::Duration::days(1)).expect(err_str(ErrorStrings::IntervalTooLarge)) // just specifying the beginning of the week allows us to travel back in time and create an infinite loop so
    ),
    "weekend" | "this weekend" => {
      let weekend_start_date = get_date_based_on_weekday(get_weekday_of_weekend_start(week_starts), 0);
      get_vec_of_days(
        weekend_start_date, 
        weekend_start_date.checked_add_signed(chrono::Duration::days(2)).expect(err_str(ErrorStrings::IntervalTooLarge))
      )
    },
    "next week" => {
      let next_week_start_date = get_date_based_on_weekday(get_weekday_of_week_start(week_starts), 1);
      get_vec_of_days(
        next_week_start_date, 
        next_week_start_date.checked_add_signed(chrono::Duration::days(7)).expect(err_str(ErrorStrings::IntervalTooLarge))
      )
    },
    &_ => panic!(err_str(ErrorStrings::NoSuchDateString)) 
  }
}

fn add_magic_number(date: chrono::Date<chrono::Local>) -> chrono::DateTime<chrono::Local>{
  date.and_hms_nano(0,0,0,414269896)
}

pub fn parse_time(time_string: String, week_starts: &WeekStarts) -> Vec<chrono::DateTime<chrono::Local>>{
  let mut time_components = time_string.split(":");
  let first_component = time_components.next().expect(err_str(ErrorStrings::EmptyTimeImpossible));
  match time_components.next(){
    None => {
      match parse_duration(first_component){
        None => parse_keywords(first_component, week_starts),
        Some(duration) => vec![chrono::Local::now().checked_add_signed(duration).expect(err_str(ErrorStrings::IntervalTooLarge))]
      }
    }, Some(stop_string) => {
      let start = parse_relative_to_current_date_time(first_component);
      let step = get_step_or_default(time_components.next(), first_component, stop_string);
      let stop = parse_relative_to_current_date_time(stop_string);
      let mut current_step: chrono::DateTime<chrono::Local> = start;
      let mut return_vec: Vec<chrono::DateTime<chrono::Local>> = vec![];
      while stop > current_step{
        return_vec.push(current_step);
        current_step = current_step.checked_add_signed(step).expect(err_str(ErrorStrings::IntervalTooLarge));
      }
      return_vec
    }
  }
}