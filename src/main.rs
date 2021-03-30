extern crate clap;
use std::fs::File;
use vcf::{VCFReader, VCFRecord};
use std::io::{BufReader};
use flate2::read::MultiGzDecoder;
use megadrile::{error, config};

fn get_vcf_reader(input: &str)
    -> Result<VCFReader<BufReader<MultiGzDecoder<File>>>, error::Error> {
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
                Err(_error) => {
                    println!("Something went wrong!")
                }
            }
        },
        None =>
            println!("No input specified.")
    }
    println!("Done!");
}
