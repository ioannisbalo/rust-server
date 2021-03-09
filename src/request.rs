use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Request {
  pub buffer: [u8; 1024],
  pub string: String,
  pub method: String,
  pub path: String,
  pub params: HashMap<String, String>
}

impl Request {
  pub fn new(buffer: [u8; 1024]) -> Self {
    let string = String::from_utf8_lossy(&buffer[..]).to_string();
    
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(?P<method>GET|POST|PUT|DELETE) (?P<path>/[a-zA-Z\d/]*)\??(?P<params>.*) .*").unwrap();
    }
    let method = get_match(&string, "method", &RE);
    let path = get_match(&string, "path", &RE);
    let params = get_match(&string, "params", &RE);

    Request {
      buffer,
      string,
      method,
      path,
      params: parse_params(&params)
    }
  }
}

fn get_match(input: &str, name: &str, regex: &Regex) -> String {
  let matched = regex.captures(input).and_then(|capture| {
    capture.name(name).map(|method| method.as_str())
  });

  if let Some(capture) = matched {
    String::from(capture)
  } else {
    String::from("")
  }
}

fn parse_params(params: &str) -> HashMap<String, String> {
  let mut param_map = HashMap::new();
  if params == String::from("") {
    return param_map;
  }

  let param_iter = params.split("&");
  let params: Vec<&str> = param_iter.collect();

  for param in params.iter() {
    let key_value: Vec<&str> = param.split("=").collect();
    let key = key_value.get(0).unwrap();
    let value = key_value.get(1).unwrap();
    param_map.insert(String::from(*key), String::from(*value));
  }

  param_map
}