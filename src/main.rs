mod config;

extern crate clap;
use std::fs::File;
use vcf::{VCFReader, VCFError, VCFRecord};
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
            println!("Input: {}", input);
            let vcf_reader = get_vcf_reader(input);
            match vcf_reader {
                Ok(mut vcf_reader) => {
                    let header = vcf_reader.header();
                    let n_samples = header.samples().len();
                    println!("Number of samples: {}", n_samples);
                    let mut record: VCFRecord = vcf_reader.empty_record();
                    let mut n_records = 0;
                    loop {
                        match vcf_reader.next_record(&mut record) {
                            Ok(got_record) => {
                                if got_record {
                                    n_records += 1;
                                } else {
                                    break
                                }
                            }
                            Err(_) => {
                                println!("Something went wrong while reading record.");
                                break
                            }
                        }
                    }
                    println!("Number of records: {}", n_records);
                }
                Err(_snag) => {
                    println!("Something went wrong!")
                }
            }
        },
        None =>
            println!("No input specified.")
    }
    println!("Done!");
}
