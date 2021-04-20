use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request,Response,StatusCode};
pub struct Server{
    ip_addr : String,
}
impl Server{
    pub fn new(ip_addr:String)->Self{
        Self{
            ip_addr
        }
    }
    pub fn run(self){
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
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("<h1>Works Properly</h1>".to_string()))
                                },
                                Err(e)=>{
                                    println!("Failed Error {}",e);
                                    Response::new(StatusCode::BadRequest, None)
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


