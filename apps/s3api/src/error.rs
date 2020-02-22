extern crate failure;

use std::fmt::Debug;
use std::string;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "clap_task::Error > {}", 0)]
    ClapTaskError(clap_task::Error),

    #[fail(display = "clap_extractor::Error > {}", 0)]
    ClapExtractorError(String),

    #[fail(display = "sabi_s3::Error > {}", 0)]
    SabiS3Error(sabi_s3::Error),

    #[fail(display = "serde_json::Error > {}", 0)]
    SerdeJsonError(serde_json::Error),

    #[fail(display = "string::FromUtf8Error > {}", 0)]
    StringFromUtf8Error(string::FromUtf8Error),
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

impl From<sabi_s3::Error> for Error {
    fn from(e: sabi_s3::Error) -> Self {
        Error::SabiS3Error(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(e)
    }
}
