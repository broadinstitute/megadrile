use crate::{error, read};
use std::io::BufWriter;
use std::fs::File;
use crate::read::VariantListWriter;

pub fn write_variant_list_file(input: &str, output: &str) -> Result<(), error::Error> {
    let mut vcf_reader = read::get_vcf_reader(input)?;
    let out_writer = BufWriter::new(File::create(output)?);
    let mut list_writer = VariantListWriter::new(out_writer);
    read::apply_record_inspector(&mut vcf_reader, &mut list_writer)
}