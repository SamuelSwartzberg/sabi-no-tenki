use ureq;

pub fn get_results_from_requests(urls: Vec<String>) -> Result<Vec<String>, ureq::Error>{ 
  let mut results: Vec<String> = vec![];
  for url in urls{
    let response = ureq::get(&url).call()?;
    results.push(response.into_string()?);
  }
  Ok(results)
} 