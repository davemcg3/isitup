extern crate reqwest;

fn main() {
  println!("{}", upordown("https://www.swellinvesting.com"));
}

pub fn upordown(site: &'static str) -> &'static str {
  let url = format!("{}", site);
  let result = reqwest::get(&url);
  if result.is_err() {
    return "site doesn't exist!";
  } else {
    let result = result.unwrap();
    println!("{}", result.status());
    if result.status() == reqwest::StatusCode::OK {
      return "site is up!";
    } else {
      return "site is down!";
    }
  }
}

