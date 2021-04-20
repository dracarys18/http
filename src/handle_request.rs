use super::server::Handler;
use std::fs;
use super::http::{Request,Response,StatusCode,Methods};
pub struct HandleRequest{
    html_path:String
}
impl HandleRequest{
    pub fn new(html_path:String)->Self{
        Self {
            html_path
        }
    }
    pub fn read_from_file(&self,file_path: &str)->Option<String>{
        let path = format!("{}/{}",self.html_path,file_path);
        match fs::canonicalize(path){
            Ok(path)=> {
                if path.starts_with(&self.html_path){
                    fs::read_to_string(path).ok()
                }
                else{
                    println!("Someone tried to get the dir {}",file_path);
                    None
                }
            },
            Err(_)=> None,
        }
    }
}
impl Handler for HandleRequest{
    fn handle_request(&mut self,req:Request)->Response{
        match req.method(){
            Methods::GET=>{
                match req.path(){
                    "/"=>Response::new(StatusCode::Ok,self.read_from_file("ex.html")),
                    path=>match self.read_from_file(path){
                        Some(fcont)=>Response::new(StatusCode::Ok,Some(fcont)),
                        None=> Response::new(StatusCode::NotFound,None)
                    },
                }
            },
            _=> Response::new(StatusCode::NotFound,None),
        }
    }
}
