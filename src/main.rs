#![allow(dead_code)]
use server::Server;
use http::Methods;
mod server;
mod http;
fn main() {
let get = Methods::GET;
let post = Methods::POST;
let put = Methods::PUT;
let delete = Methods::DELETE;
let ser = Server::new("127.0.0.1:8080".to_string());
ser.run();   
}
