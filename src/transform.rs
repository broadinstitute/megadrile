use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::read::{self, VariantListWriter};

pub fn write_variant_list_file(input: &str, output: &str) -> crate::Result<()> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let out_writer = BufWriter::new(File::create(output)?);
    let mut list_writer = VariantListWriter::new(out_writer);
    read::apply_record_inspector(&mut vcf_reader, &mut list_writer)
}

pub fn write_sample_list_file(input: &str, output: &str) -> crate::Result<()> {
    let vcf_reader = read::get_vcf_reader(input)?;
    let mut out_writer = BufWriter::new(File::create(output)?);
    for sample in vcf_reader.header().samples() {
        out_writer.write(sample.as_slice())?;
        out_writer.write(b"\n")?;
    }
    Ok(())
}
