use std::sync::Arc;
use std::collections::HashMap;

use crate::{Route, Handler};

pub struct RouteService {
  get: HashMap<String, Arc<Route>>,
  post: HashMap<String, Arc<Route>>,
  put: HashMap<String, Arc<Route>>,
  delete: HashMap<String, Arc<Route>>,
  not_found: Arc<Route>
}

impl RouteService {
  pub fn new() -> Self {
    RouteService {
      get: HashMap::new(),
      post: HashMap::new(),
      put: HashMap::new(),
      delete: HashMap::new(),
      not_found: Arc::new(Route {
        was_found: false,
        handler: Box::new(|_| String::from("Not found"))
      })
    }
  }

  pub fn register_route(&mut self, method: &str, path: &str, handler: Handler) -> Result<(), &str> {
    let map = self.get_map(method).ok_or("Not existing Method")?;

    if map.contains_key(path) {
      Err("Path already registered for Method")
    } else {
      let route = Arc::new(Route { handler, was_found: true });
      map.insert(String::from(path), route);
      Ok(())
    }

  }

  pub fn register_not_found(&mut self, handler: Handler) {
    self.not_found = Arc::new(Route {
      handler,
      was_found: false
    });
  }

  pub fn get_route(&self, method: &str, path: &str) -> Arc<Route> {
    let map = match method {
      "GET" => &self.get,
      "POST" => &self.post,
      "PUT" => &self.put,
      "DELETE" => &self.delete,
      _ => return Arc::clone(&self.not_found)
    };

    if let Some(route) = &map.get(path) {
      Arc::clone(&route)
    } else {
      Arc::clone(&self.not_found)
    }
  }

  fn get_map(&mut self, method: &str) -> Option<&mut HashMap<String, Arc<Route>>> {
    match method {
      "GET" => Some(&mut self.get),
      "POST" => Some(&mut self.post),
      "PUT" => Some(&mut self.put),
      "DELETE" => Some(&mut self.delete),
      _ => return None
    }
  }
}
