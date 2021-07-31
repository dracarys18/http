use crate::handle_request::HandleRequest;
use crate::http::Request;
use std::convert::TryFrom;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
pub struct Server {
    ip_addr: String,
}

impl Server {
    pub fn new(ip_addr: String) -> Self {
        Self { ip_addr }
    }
    pub async fn run(self, mut handler: HandleRequest) {
        println!("Listening on {}", self.ip_addr);
        let listener = TcpListener::bind(&self.ip_addr).await.unwrap();
        loop {
            match listener.accept().await {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer).await {
                        Ok(_) => {
                            println!("Request received {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(request).await,
                                Err(e) => {
                                    println!("Failed Error {}", e);
                                    handler.handle_for_badrequest(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream).await {
                                println!("Failed Error {}", e);
                            }
                        }
                        Err(e) => println!("Error {}", e),
                    }
                }
                Err(e) => println!("Cannot listen because of the error {}", e),
            }
        }
    }
}
