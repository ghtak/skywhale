use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{
    #[error("Io Error {0}")]
    IoError(#[from] io::Error)
}