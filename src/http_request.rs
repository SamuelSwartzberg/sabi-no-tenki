use ureq;
use serde_json;

pub fn get_results_from_requests(urls: &Vec<String>) -> Result<Vec<serde_json::Value>, ureq::Error>{ 
  let mut results: Vec<serde_json::Value> = vec![];
  for url in urls{
    let response = ureq::get(&url).call()?;
    results.push(response.into_json()?);
  }
  Ok(results)
} 