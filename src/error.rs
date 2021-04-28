use std::io;
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

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::MDError(string) => { string.to_string() }
            Error::Io(io_error) => { format!("{:?}", io_error) }
            Error::Vcf(vcf_error) => { vcf_error.to_string() }
            Error::Utf8(utf8_error) => { utf8_error.to_string() }
        }
    }
}
