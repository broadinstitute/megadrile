use std::{
    fs::File,
    io::{self, BufReader, Write},
};

use flate2::read::MultiGzDecoder;
use vcf::{VCFReader, VCFRecord};

use crate::error::Error;

pub trait VcfRecordInspector<R> {
    fn inspect_record(&mut self, record: &VCFRecord) -> crate::Result<()>;
    fn get_result(&mut self) -> crate::Result<R>;
}

pub fn get_vcf_reader(input: &str) -> crate::Result<VCFReader<BufReader<MultiGzDecoder<File>>>> {
    Ok(VCFReader::new(BufReader::new(MultiGzDecoder::new(
        File::open(input)?,
    )))?)
}

pub struct RecordCounter {
    n_records: u32,
}

impl RecordCounter {
    pub fn new() -> RecordCounter {
        RecordCounter { n_records: 0 }
    }
}

impl VcfRecordInspector<u32> for RecordCounter {
    fn inspect_record(&mut self, _record: &VCFRecord) -> crate::Result<()> {
        self.n_records += 1;
        Ok(())
    }

    fn get_result(&mut self) -> crate::Result<u32> {
        Ok(self.n_records)
    }
}

pub fn apply_record_inspector<B: io::BufRead, R, I: VcfRecordInspector<R>>(
    reader: &mut VCFReader<B>,
    inspector: &mut I,
) -> crate::Result<R> {
    let mut record = reader.empty_record();
    loop {
        let has_record = reader.next_record(&mut record)?;
        if has_record {
            inspector.inspect_record(&record)?;
        } else {
            break inspector.get_result();
        }
    }
}

pub struct VariantListWriter<W: Write> {
    write: W,
}

impl<W: Write> VariantListWriter<W> {
    pub fn new(write: W) -> VariantListWriter<W> {
        VariantListWriter { write }
    }
}

impl<W: Write> VcfRecordInspector<()> for VariantListWriter<W> {
    fn inspect_record(&mut self, record: &VCFRecord) -> crate::Result<()> {
        for id in &record.id {
            self.write.write(id)?;
            self.write.write(b"\n")?;
        }
        Ok(())
    }

    fn get_result(&mut self) -> crate::Result<()> {
        self.write.flush()?;
        Ok(())
    }
}

pub struct MafWriter<W: Write> {
    write: W,
}

impl<W: Write> MafWriter<W> {
    pub fn new(write: W) -> MafWriter<W> {
        MafWriter { write }
    }
}

const KEY_GT: &[u8; 2] = b"GT";

impl<W: Write> VcfRecordInspector<()> for MafWriter<W> {
    fn inspect_record(&mut self, record: &VCFRecord) -> Result<(), Error> {
        static GENOTYPE_SPLIT_CHARACTERS: &[char] = &['|', '/'];

        // let alts: Vec<&Vec<u8>> = record.header().alt_list().collect();
        let alts = &record.alternative;
        println!("alts.len() == {}", alts.len());
        let mut alt_counts = vec![0u64; alts.len()];
        for sample in record.header().samples() {
            if let Some(genotypes) = record.genotype(sample, KEY_GT) {
                for genotype_bytes in genotypes {
                    let genotype = std::str::from_utf8(genotype_bytes)?;
                    for i_allele_str in genotype.split(GENOTYPE_SPLIT_CHARACTERS) {
                        if let Ok(i_alt) = i_allele_str.parse::<usize>() {
                            if i_alt > 0 {
                                alt_counts[i_alt - 1] += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("alts.len() = {}", alts.len());
        for i in 0..alts.len() {
            let alt = &alts[i];
            let alt_count = alt_counts[i];
            println!("alt = {}", std::str::from_utf8(alt).unwrap());
            println!("alt_count = {}", alt_count);
            let mut is_first = true;
            for id in &record.id {
                if is_first {
                    is_first = false
                } else {
                    self.write.write(b", ")?;
                }
                self.write.write(&id)?;
            }
            self.write.write(b"\t")?;
            self.write.write(&alt)?;
            self.write.write(b"\t")?;
            self.write.write(alt_count.to_string().as_bytes())?;
            self.write.write(b"\n")?;
        }
        Ok(())
    }

    fn get_result(&mut self) -> crate::Result<()> {
        self.write.flush()?;
        Ok(())
    }
}
