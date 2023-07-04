use std::io;
use tracing::debug;

use error::ErrorCode;
use crate::utils::init_tracing;

mod error;
mod utils;
mod tokio_echo;
mod future;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let _guard = init_tracing();
    let ec: ErrorCode = io::Error::new(io::ErrorKind::AddrInUse, "Extra Text").into();
    debug!("{}", ec);
    future::local_main();
}
