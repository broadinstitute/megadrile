extern crate clap;

use std::string;
use megadrile::{config, error};
use megadrile::read::{self, RecordCounter};

struct Stats {
    n_samples: u32,
    n_records: u32
}

fn count_samples_and_records(input: &str) -> Result<Stats, error::Error> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let n_samples = vcf_reader.header().samples().len() as u32;
    let mut record_counter = RecordCounter::new();
    let n_records =
        read::apply_record_inspector(&mut vcf_reader, &mut record_counter)?;
    Ok(Stats { n_samples, n_records })
}

fn try_get_stats() -> Result<Stats, error::Error> {
    let cli_config = config::get_cli_config();
    let input = cli_config.value_of("input")
        .ok_or(error::Error::MDError(string::String::from("No input given.")))?;
    count_samples_and_records(input)
}

fn main() {
    match try_get_stats() {
        Ok(stats) => {
            println!("Number of samples: {}.", stats.n_samples);
            println!("Number of records: {}.", stats.n_records);
        }
        Err(_) => {
            println!("Error!")
        }
    }
    println!("Done!");
}
