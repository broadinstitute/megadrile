extern crate clap;

use megadrile::config;
use megadrile::read::{self, RecordCounter};

fn main() {
    let cli_config = config::get_cli_config();
    match cli_config.value_of("input") {
        Some(input) => {
            println!("Input: {}", input);
            let vcf_reader = read::get_vcf_reader(input);
            match vcf_reader {
                Ok(mut vcf_reader) => {
                    let header = vcf_reader.header();
                    let n_samples = header.samples().len();
                    println!("Number of samples: {}", n_samples);
                    let mut record_counter = RecordCounter::new();
                    match read::apply_record_inspector(&mut vcf_reader, &mut record_counter) {
                        Ok(n_records) => {
                            println!("Number of records: {}", n_records)
                        }
                        Err(_) => {
                            println!("Something went wrong while reading records.");
                        }
                    }
                }
                Err(_error) => {
                    println!("Something went wrong!")
                }
            }
        }
        None =>
            println!("No input specified.")
    }
    println!("Done!");
}
