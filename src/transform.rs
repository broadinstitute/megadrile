use std::{fs::File, io::{BufWriter, Write}};

use crate::{error, read::{self, VariantListWriter}};

pub fn write_variant_list_file(input: &str, output: &str) -> Result<(), error::Error> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let out_writer = BufWriter::new(File::create(output)?);
    let mut list_writer = VariantListWriter::new(out_writer);
    read::apply_record_inspector(&mut vcf_reader, &mut list_writer)
}

pub fn write_sample_list_file(input: &str, output: &str) -> Result<(), error::Error> {
    let vcf_reader = read::get_vcf_reader(input)?;
    let mut out_writer = BufWriter::new(File::create(output)?);
    for sample in vcf_reader.header().samples().iter() {
        out_writer.write(sample.as_slice())?;
        out_writer.write("\n".as_bytes())?;
    }
    Ok(())
}

