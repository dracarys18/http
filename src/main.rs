#![allow(dead_code)]
use server::Server;
use http::Methods;
use std::env;
use handle_request::HandleRequest;
mod server;
mod http;
mod handle_request;
fn main() {
let default_path=format!("{}/html",env!("CARGO_MANIFEST_DIR"));
let html_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
println!("Current path is {}",html_path);
let ser = Server::new("127.0.0.1:8080".to_string());
ser.run(HandleRequest::new(html_path));   
}
