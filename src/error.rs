use std::{io, convert, string};
use vcf::VCFError;

pub enum Error {
    MDError(string::String),
    Io(io::Error),
    Vcf(VCFError),
}

impl convert::From<vcf::VCFError> for Error {
    fn from(vcf_error: VCFError) -> Error {
        Error::Vcf(vcf_error)
    }
}

impl convert::From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::Io(io_error)
    }
}

impl convert::From<&str> for Error {
    fn from(message: &str) -> Error { Error::MDError(string::String::from(message)) }
}

impl convert::From<string::String> for Error {
    fn from(message: string::String) -> Error { Error::MDError(message) }
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::MDError(string) => { string.to_string() }
            Error::Io(io_error) => { format!("{:?}", io_error) }
            Error::Vcf(vcf_error) => { vcf_error.to_string() }
        }
    }
}
