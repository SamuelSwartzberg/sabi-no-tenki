use strum;
use strum::EnumMessage;
use strum_macros::EnumString;

#[derive(Debug, strum_macros::EnumMessage)]
pub enum ErrorStrings{
  #[strum(message = "API expected but none provided!")]
  NoApi,
  #[strum(message = "The specified API does not exist. Did you specify an extant API, or perhaps misspell the name?")]
  NoSuchApi,
  #[strum(message = "The specified date string does not exist. Did you perhaps misspell the name?")]
  NoSuchDateString,
  #[strum(message = "Specified number not parsable. Number provides was for ")]
  NotAParsableNumber,
  #[strum(message = "The date you specified resulted in a date that is impossible")]
  NoSuchTime,
  #[strum(message = "")]
  TimeFormatError,
  #[strum(message = "Specified metric does not exist. Metric supplied was ")]
  NoSuchMetric
}