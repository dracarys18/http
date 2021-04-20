use std::str::FromStr;
#[derive(Debug)]
pub enum Methods{
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
impl FromStr for Methods{
    type Err = MethodErr;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s{
            "GET"=>Ok(Self::GET),
            "POST"=>Ok(Self::POST),
            "PUT"=>Ok(Self::PUT),
            "DELETE"=>Ok(Self::DELETE),
            "HEAD"=>Ok(Self::HEAD),
            "CONNECT"=>Ok(Self::CONNECT),
            "OPTIONS"=>Ok(Self::OPTIONS),
            "TRACE"=>Ok(Self::TRACE),
            "PATCH"=>Ok(Self::PATCH),
            _=>Err(MethodErr),  
        }
    }
}

pub struct MethodErr;

