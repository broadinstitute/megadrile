use clap::ArgMatches;
use crate::{error, transform};
use crate::stats;

fn get_str<'a>(arg_matches: &'a ArgMatches, name: &str) -> Result<&'a str, error::Error> {
    arg_matches.value_of(name)
        .ok_or(error::Error::MDError(format!("Missing argument {}.", name)))
}

pub fn print_counts(sub_matches: &ArgMatches) -> Result<(), error::Error> {
    let input = get_str(sub_matches, "input")?;
    let counts = stats::count_samples_and_records(input)?;
    println!("Number of samples: {}.", counts.n_samples);
    println!("Number of records: {}.", counts.n_records);
    Ok(())
}

pub fn write_list_of_variants(sub_matches: &ArgMatches) -> Result<(), error::Error> {
    let input = get_str(sub_matches, "input")?;
    let output = get_str(sub_matches, "output")?;
    println!("Reading from {} and writing variants to {}", input, output);
    transform::write_variant_list_file(input, output)
}

pub fn write_list_of_samples(sub_matches: &ArgMatches) -> Result<(), error::Error> {
    let input = get_str(sub_matches, "input")?;
    let output = get_str(sub_matches, "output")?;
    println!("Reading from {} and writing samples to {}", input, output);
    transform::write_sample_list_file(input, output)
}

pub fn calculate_maf(sub_matches: &ArgMatches) -> Result<(), error::Error> {
    let input = get_str(sub_matches, "input")?;
    let output = get_str(sub_matches, "output")?;
    println!("Reading from {} and writing MAF to {}", input, output);
    stats::write_maf(input, output)
}

