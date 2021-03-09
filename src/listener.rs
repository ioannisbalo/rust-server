use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;
use std::sync::Arc;

use crate::{Connection, ThreadPool, Configuration, RouteService, Route, Request, Handler};

enum ListenerState {
  Waiting,
  Started
}

pub struct Listener {
  config: Configuration,
  state: ListenerState,
  route_service: RouteService,
  pool: ThreadPool
}

impl Listener {
  pub fn new(config: Configuration) -> Self {
    let pool = ThreadPool::new(config.thread_number);
    let route_service = RouteService::new();

    Listener { config, state: ListenerState::Waiting, route_service, pool }
  }

  pub fn start(&mut self) -> Result<(), &str> {
    if let ListenerState::Started = self.state {
      return Err("Server already started")
    }

    let url = format!("{}:{}", self.config.host, self.config.port);
    let listener = TcpListener::bind(url).unwrap();
    self.state = ListenerState::Started;
  
    for stream in listener.incoming() {
      let stream = stream.unwrap();
      let (connection, request) = get_connection_info(stream);
      let route = self.route_service.get_route(&request.method, &request.path);

      self.pool.execute(Box::new(move || {
        handle_connection(connection, request, route);
      }));
    }

    Ok(())
  }

  pub fn register_route(&mut self, method: &str, path: &str, handler: Handler) -> Result<(), &str> {
    self.route_service.register_route(method, path, handler)
  }

  pub fn register_not_found(&mut self, handler: Handler) {
    self.route_service.register_not_found(handler);
  }
}

fn get_connection_info(mut stream: TcpStream) -> (Connection, Request) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let request = Request::new(buffer);
  let connection = Connection::new(stream);

  (connection, request)
}

fn handle_connection(mut connection: Connection, request: Request, route: Arc<Route>) {
  let handler = &route.handler;
  println!("Request: {}", request.string);

  if route.was_found {
    connection.respond(&handler(&request));
  } else {
    connection.not_found(&handler(&request));
  }
}