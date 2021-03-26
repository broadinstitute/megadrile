mod config;

extern crate clap;
use std::fs::File;
use vcf::{VCFReader, VCFError};
use std::io::{BufReader};
use std::io;
use flate2::read::MultiGzDecoder;
use std::convert;

enum Snag {
    IoSnag(io::Error),
    VCFSnag(VCFError)
}

impl convert::From<vcf::VCFError> for Snag {
    fn from(error: VCFError) -> Snag {
        Snag::VCFSnag(error)
    }
}

impl convert::From<io::Error> for Snag {
    fn from(error: io::Error) -> Snag {
        Snag::IoSnag(error)
    }
}

fn get_vcf_reader(input: &str) -> Result<VCFReader<BufReader<MultiGzDecoder<File>>>, Snag> {
    Ok(VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(input)?)))?)
}

fn main() {
    let cli_config = config::get_cli_config();
    match cli_config.value_of("input") {
        Some(input) => {
            let mut vcf_reader = get_vcf_reader(input);
            println!("Input: {}", input)
        },
        None =>
            println!("No input specified.")
    }
    println!("Done!");
}
