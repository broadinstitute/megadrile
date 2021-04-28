use std::{fs::File, io::BufWriter};

use crate::{error, read::{self, RecordCounter, MafWriter}};

pub struct Counts {
    pub n_samples: u32,
    pub n_records: u32
}

pub fn count_samples_and_records(input: &str) -> Result<Counts, error::Error> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let n_samples = vcf_reader.header().samples().len() as u32;
    let mut record_counter = RecordCounter::new();
    let n_records =
        read::apply_record_inspector(&mut vcf_reader, &mut record_counter)?;
    Ok(Counts { n_samples, n_records })
}

pub fn write_maf(input: &str, output: &str) -> Result<(), error::Error> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let out_writer = BufWriter::new(File::create(output)?);
    let mut maf_writer = MafWriter::new(out_writer);
    read::apply_record_inspector(&mut vcf_reader, &mut maf_writer)
}
