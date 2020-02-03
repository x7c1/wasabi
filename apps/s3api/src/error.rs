extern crate failure;

use std::fmt::Debug;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "clap_task::Error > {}", 0)]
    ClapTaskError(clap_task::Error),

    #[fail(display = "clap_extractor::Error > {}", 0)]
    ClapExtractorError(String),
}

impl From<clap_task::Error> for Error {
    fn from(e: clap_task::Error) -> Self {
        Error::ClapTaskError(e)
    }
}

impl<A: Debug> From<clap_extractor::Error<A>> for Error {
    fn from(e: clap_extractor::Error<A>) -> Self {
        Error::ClapExtractorError(format!("{:?}", e))
    }
}