use std::io;
use tracing::debug;

use error::ErrorCode;
use crate::utils::init_tracing;

mod error;
mod utils;
mod tokio_echo;

fn main() {
    let _guard = init_tracing();
    let ec: ErrorCode = io::Error::new(io::ErrorKind::AddrInUse, "Extra Text").into();
    debug!("{}", ec);
    //tokio_echo::local_main();
}
