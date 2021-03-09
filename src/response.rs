use std::fmt::{ Display, Formatter };
use std::fmt;

pub struct Response {
  code: u32,
  reason: Reason,
  body: String,
  length: usize
}

impl Response {
  pub fn new(code: u32, reason: Reason, body: String) -> Self {
    let length = body.len();

    Response {
      code,
      reason,
      body,
      length
    }
  }

  pub fn value(&self) -> String {
    format!(
      "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
      self.code,
      self.reason,
      self.length,
      self.body
    )
  }
}

pub enum Reason {
  OK,
  NotFound
}

impl Display for Reason {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let printable = match *self {
      Reason::OK => "OK",
      Reason::NotFound => "NOT FOUND"
    };
    write!(f, "{}", printable)
  }
}
