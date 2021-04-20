use super::{Methods,MethodErr};
use super::QueryString;
use std::str;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult,Formatter,Display,Debug};

#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Methods,
}
pub enum ParseError{
    InvalidRequest,
    InvalidMethod,
    InvalidEncoder,
    InvalidProtocol,
}
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f,"{}",self.message())
    } 
}
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f,"{}",self.message())
    } 
}
impl ParseError{
    fn message(&self)->&str{
        match self{
        Self::InvalidRequest=>"Invalid Request",
        Self::InvalidMethod=>"Invalid Method",
        Self::InvalidProtocol=>"Invalid Protocol",
        Self::InvalidEncoder=>"Invalid Encoder",
        }
    }
}
impl Error for ParseError{

}

impl From<MethodErr> for ParseError{
    fn from(_:MethodErr)->Self{
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoder
    }
    
}
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;
    fn try_from(value:&'buf [u8])->Result<Request<'buf>,Self::Error>{
        let request = str::from_utf8(&value)?;
        let (method,request) = nextword(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request)  = nextword(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = nextword(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        let method: Methods = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find("?"){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        Ok(
            Self{
                path,
                query_string,
                method
            }
        )
       }
}
fn nextword(val:&str)->Option<(&str,&str)>{
    for (i,c) in val.chars().enumerate(){
        if c==' ' || c == '\r'{
            return Some((&val[..i],&val[i+1..]));
        }
    }
    None
}
