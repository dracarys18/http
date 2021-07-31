use super::StatusCode;
use tokio::io::{AsyncWriteExt, Result};
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    pub async fn send(&self, stream: &mut tokio::net::TcpStream) -> Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => self.status_code.reason_phrase(),
        };
        stream
            .write_all(
                format!(
                    "HTTP/1.1 {} {} \r\n\r\n{}",
                    self.status_code,
                    self.status_code.reason_phrase(),
                    body
                )
                .as_bytes(),
            )
            .await
    }
}
