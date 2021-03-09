use std::net::TcpStream;
use std::io::prelude::*;
use crate::response::{ Response, Reason };

pub struct Connection {
  stream: TcpStream
}

impl Connection {
  pub fn new(stream: TcpStream) -> Self {
    Connection {
      stream
    }
  }

  pub fn respond(&mut self, body: &str) {
    let response = Response::new(200, Reason::OK, String::from(body));
    self.write_and_flush(response);
  }

  pub fn error(&mut self, code: u32, reason: Reason, body: &str) {
    let response = Response::new(code, reason, String::from(body));
    self.write_and_flush(response);
  }

  pub fn not_found(&mut self, body: &str) {
    self.error(404, Reason::NotFound, body);
  }

  fn write_and_flush(&mut self, response: Response) {
    self.stream.write(response.value().as_bytes()).unwrap();
    self.stream.flush().unwrap();
  }
}
