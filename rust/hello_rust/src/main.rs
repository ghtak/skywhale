mod simple_http_server;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use simple_http_server::run_server;

mod nonnull;
use nonnull::address_check;

mod error;
use error::ErrorCode;


fn main(){
    //println!("{:?}", ErrorCode::IoError(Error::from(io::ErrorKind::Interrupted)));
    println!("{:?}", error::error_into());
}
