/*use strum;*/
use strum::EnumMessage;
use strum_macros::EnumMessage;

#[derive(Debug, strum::EnumMessage, strum_macros::EnumMessage)]
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
  #[strum(message = "Time provided, but somehow resulted in an empty String.")]
  EmptyTimeImpossible,
  #[strum(message = "In calculating the relevant times, an interval was too large and resulted in an overflow.")]
  IntervalTooLarge,
  #[strum(message = "Specified metric does not exist. Metric supplied was ")]
  NoSuchMetric,
  #[strum(message = "The API returned a weather type which does not correspond to any local weather type.")]
  NoSuchWeatherType,
  #[strum(message = "Could not create path for cache directory.")]
  CouldNotCreateCachePath,
  #[strum(message = "Could not create cache file at cache path.")]
  CouldNotCreateCacheFile,
  #[strum(message = "Time list was not parseable, contains error.")]
  CouldNotParseTimeList,
  #[strum(message = "Could not calculate the date of the start or end for the specified time specifier. This should not be able to happen and is most likely a bug.")]
  CouldNotGetDateWeekday,
  #[strum(message = "Something went wrong while trying to fetch the data.")]
  HTTPError,
  #[strum(message = " was expected in this location while parsing the result from the API, but something else was found.")]
  NotInExpectedPlace,
  #[strum(message = " was found in the expeced location, but was not of the expected type.")]
  NotOfExpectedType
  #[strum(message = "Error while attempting to write to cache.")]
  CacheWriteFail
}

pub fn err_str(error_string: ErrorStrings) -> &'static str{
  error_string.get_message().expect("While getting the error message, an error occurred.")
}