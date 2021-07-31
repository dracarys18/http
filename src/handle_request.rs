use super::http::{Methods, Request, Response, StatusCode};
use tokio::fs;
pub struct HandleRequest {
    html_path: String,
}
impl HandleRequest {
    pub fn new(html_path: String) -> Self {
        Self { html_path }
    }
    pub async fn read_from_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.html_path, file_path);
        match fs::canonicalize(path).await {
            Ok(path) => {
                if path.starts_with(&self.html_path) {
                    fs::read_to_string(path).await.ok()
                } else {
                    println!("Someone tried to get the dir {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
    pub async fn handle_request(&mut self, req: Request<'_>) -> Response {
        match req.method() {
            Methods::GET => match req.path() {
                "/" => Response::new(StatusCode::Ok, self.read_from_file("ex.html").await),
                path => match self.read_from_file(path).await {
                    Some(fcont) => Response::new(StatusCode::Ok, Some(fcont)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
    pub fn handle_for_badrequest(&mut self, e: &crate::http::ParseError) -> Response {
        println!("Failed because of {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
