use std::{fmt::Display, io};
use vcf::VCFError;

#[derive(Debug)]
pub enum Error {
    MDError(String),
    Io(io::Error),
    Vcf(VCFError),
    Utf8(std::str::Utf8Error)
}

impl From<vcf::VCFError> for Error {
    fn from(vcf_error: VCFError) -> Error {
        Error::Vcf(vcf_error)
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::Io(io_error)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Error { Error::MDError(String::from(message)) }
}

impl From<String> for Error {
    fn from(message: String) -> Error { Error::MDError(message) }
}

impl From<std::str::Utf8Error> for Error {
    fn from(utf8_error: std::str::Utf8Error) -> Error { Error::Utf8(utf8_error) }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MDError(e) => f.write_str(e),
            Error::Io(e) => e.fmt(f),
            Error::Vcf(e) => e.fmt(f),
            Error::Utf8(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
