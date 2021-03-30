use std::{io, convert};
use vcf::VCFError;

pub enum Error {
    Io(io::Error),
    Vcf(VCFError)
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

