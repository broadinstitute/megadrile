use crate::read::RecordCounter;
use crate::{read, error};
use std::string;
use clap::ArgMatches;

pub struct Counts {
    pub n_samples: u32,
    pub n_records: u32
}

fn count_samples_and_records(input: &str) -> Result<Counts, error::Error> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let n_samples = vcf_reader.header().samples().len() as u32;
    let mut record_counter = RecordCounter::new();
    let n_records =
        read::apply_record_inspector(&mut vcf_reader, &mut record_counter)?;
    Ok(Counts { n_samples, n_records })
}

pub fn try_get_counts(arg_matches: &ArgMatches) -> Result<Counts, error::Error> {
    let input = arg_matches.value_of("input")
        .ok_or(error::Error::MDError(string::String::from("No input given.")))?;
    count_samples_and_records(input)
}


