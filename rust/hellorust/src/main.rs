use std::io;

use error::ErrorCode;

mod error;

fn main() {
    let ec: ErrorCode = io::Error::new(io::ErrorKind::AddrInUse, "Extra Text").into();
    println!("{}", ec);
}
