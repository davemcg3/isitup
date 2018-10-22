extern crate reqwest;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let result_status;
  let mut response = Response::new("", &reqwest::StatusCode::CONTINUE, "");
  
  //let site: &str;
  if args.len() >= 2 {
    response.site = &args[1];
  } else {
    response.site = "https://www.swellinvesting.com";
  }
  println!("{}", response.site);

  //let site_status: &str;
  
  let url = format!("{}", response.site);
  let result = reqwest::get(&url);
  if result.is_err() {
    response.site_message = "site doesn't exist!";
  } else {
    let result = result.unwrap();
    println!("{}", result.status());
    result_status = &result.status();
    response.site_status = result_status;
    if result.status() == reqwest::StatusCode::OK {
      response.site_message = "site is up!";
    } else {
      response.site_message = "site is down!";
    }
  }
  println!("{}", response.site_status);
}

pub struct Response<'a> {
  pub site: &'a str,
  pub site_status: &'a reqwest::StatusCode,
  pub site_message: &'a str,
}

impl<'a> Response<'a> {
  pub fn new(site: &'static str, site_status: &'a reqwest::StatusCode, site_message: &'static str) -> Response<'a> {
    Response { site, site_status, site_message }
  }
}

//pub fn run(response: Response) -> Result<Response, &'static str> {
//  return "oogie boogie 2";
//}
