use ureq;

pub fn get_results_from_requests(urls: Vec<String>) -> Option<Vec<String>>{ 
  let mut results: Vec<String> = vec![];
  for url in urls{
    let response = ureq::get(&url).call().ok()?;
    results.push(response.into_string().ok()?);
  }
  Some(results)
} 