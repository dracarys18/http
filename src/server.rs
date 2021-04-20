use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request,Response,StatusCode,ParseError};
pub struct Server{
    ip_addr : String,
}
pub trait Handler{
    fn handle_request(&mut self,req:Request)->Response;
    fn handle_for_badrequest(&mut self,e:&ParseError)->Response{
        println!("Failed because of {}",e);
        Response::new(StatusCode::BadRequest,None)
    }
}
impl Server{
    pub fn new(ip_addr:String)->Self{
        Self{
            ip_addr
        }
    }
    pub fn run(self,mut handler:impl Handler){
        println!("Listening on {}",self.ip_addr); 
        let listener = TcpListener::bind(&self.ip_addr).unwrap();
        loop{
            match listener.accept(){
                Ok((mut stream,_))=>{
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer){
                        Ok(_)=>{
                            println!("Request received {}",String::from_utf8_lossy(&buffer));
                            let response= match Request::try_from(&buffer[..]){
                                Ok(request)=>{
                                    handler.handle_request(request)
                                },
                                Err(e)=>{
                                    println!("Failed Error {}",e);
                                    handler.handle_for_badrequest(&e)
                                },
                            };
                            if let Err(e)= response.send(&mut stream){
                                println!("Failed Error {}",e);
                            }
                        }
                        Err(e)=>println!("Error {}",e),
                    }
                }
                Err(e)=>println!("Cannot listen because of the error {}",e),
            }
        }  
    }
}


