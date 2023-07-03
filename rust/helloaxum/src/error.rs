use std::io;
use thiserror::Error;


#[derive(Error, Debug)]
pub(crate) enum ErrorCode{
    #[error("io::Error {0:?}",)]
    IoError(#[from] io::Error),
}