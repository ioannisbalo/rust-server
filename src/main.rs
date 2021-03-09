use server::{Listener, Configuration, Request, Handler};
use std::fs;
use lazy_static::lazy_static;

fn main() {
  lazy_static! {
    static ref HELLO: String = fs::read_to_string("hello.html").unwrap();
    static ref NOT_FOUND: String = fs::read_to_string("404.html").unwrap();
  }

  let config: Configuration = Configuration{
    host: String::from("127.0.0.1"),
    port: String::from("4000"),
    thread_number: 4
  };
  let mut listener = Listener::new(config);
  
  listener.register_route("GET", "/", hello(&HELLO)).unwrap();
  listener.register_route("GET", "/hello", hello(&HELLO)).unwrap();
  listener.register_not_found(not_found(&NOT_FOUND));

  listener.start().unwrap();
}

fn hello(page: &'static str) -> Handler {
  let handler = move |request: &Request| {
    if let Some(name) = request.params.get("name") {
      println!("Hello {}!!", name);
    } else {
      println!("Hello anonymous!!");
    }

    String::from(page)
  };

  Box::new(handler)
}

fn not_found(page: &'static str) -> Handler {
  let handler = move |_: &Request| {
    String::from(page)
  };

  Box::new(handler)
}
