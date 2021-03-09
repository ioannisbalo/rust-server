# rust-server

## Description
A basic implementation of an HTTP server in rust.

## Use
* Import the necessary items from the library:
`use server::{Listener, Configuration, Request, Handler};`
* Create an instance of Configuration with the host, port and maximum threads to be used:
```
let config: Configuration = Configuration{
  host: String::from("127.0.0.1"),
  port: String::from("4000"),
  thread_number: 4
};
```
* Create an instance of Listener: `let mut listener = Listener::new(config);`
* Register routes to be handled by providing the path and a function / closure that implements the Handler type (`Box<dyn Send + Sync + Fn(&Request) -> String>`):
```
listener.register_route("GET", "/goodbye", Box::new(goodbye)).unwrap();
```
and
```
fn goodbye(_: &Request) -> String {
  String::from("Goodbye!!")
}
```
* Start listening for requests: `listener.start().unwrap();`

Check main.rs for more examples.

## TODO
* Parse and provide body as part of the Request.
* Include more request info in the Request (headers, etc).
