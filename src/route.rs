use crate::Request;

// pub type Handler = fn(&Request) -> String;
pub type Handler = Box<dyn Send + Sync + Fn(&Request) -> String>;

pub struct Route {
  pub was_found: bool,
  pub handler: Handler
}
