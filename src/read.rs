use vcf::{VCFRecord, VCFReader};
use crate::error;
use flate2::read::MultiGzDecoder;
use std::io::{BufReader, Write};
use std::io;
use std::fs::File;
use crate::error::Error;

pub trait VcfRecordInspector<R> {
    fn inspect_record(&mut self, record: &VCFRecord) -> Result<(), error::Error>;
    fn get_result(&mut self) -> Result<R, error::Error>;
}

pub fn get_vcf_reader(input: &str)
                      -> Result<VCFReader<BufReader<MultiGzDecoder<File>>>, error::Error> {
    Ok(
        VCFReader::new(
            BufReader::new(MultiGzDecoder::new(File::open(input)?))
        )?
    )
}

pub struct RecordCounter {
    n_records: u32
}

impl RecordCounter {
    pub fn new() -> RecordCounter {
        RecordCounter {
            n_records: 0
        }
    }
}

impl VcfRecordInspector<u32> for RecordCounter {
    fn inspect_record(&mut self, _record: &VCFRecord) -> Result<(), error::Error> {
        self.n_records += 1;
        Ok(())
    }

    fn get_result(&mut self) -> Result<u32, error::Error> {
        Ok(self.n_records)
    }
}

pub fn apply_record_inspector<B: io::BufRead, R, I: VcfRecordInspector<R>>
(reader: &mut VCFReader<B>, inspector: &mut I) -> Result<R, error::Error> {
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
    write: W
}

impl<W: Write> VariantListWriter<W> {
    pub fn new(write: W) -> VariantListWriter<W> {
        VariantListWriter { write }
    }
}

impl<W: Write> VcfRecordInspector<()> for VariantListWriter<W> {
    fn inspect_record(&mut self, record: &VCFRecord) -> Result<(), error::Error> {
        for id in record.id.iter() {
            let id_bytes: &[u8] = id;
            self.write.write(id_bytes)?;
            self.write.write("\n".as_bytes())?;
        }
        Ok(())
    }

    fn get_result(&mut self) -> Result<(), error::Error> {
        self.write.flush()?;
        Ok(())
    }
}

pub struct MafWriter<W: Write> {
    write: W
}

impl<W: Write> MafWriter<W> {
    pub fn new(write: W) -> MafWriter<W> {
        MafWriter { write }
    }
}

const KEY_GT: &[u8; 2] = b"GT";

impl<W: Write> VcfRecordInspector<()> for MafWriter<W> {
    fn inspect_record(&mut self, record: &VCFRecord) -> Result<(), Error> {
        let alts: Vec<&Vec<u8>> = record.header().alt_list().collect();
        let mut alt_counts = vec![0u64; alts.len()];
        for sample in record.header().samples() {
            if let Some(genotypes) = record.genotype(sample, KEY_GT) {
                for genotype in genotypes {
                    if genotype.len() == 1 {
                        let alt = genotype[0];
                        //  We're assuming there are no more than 10 alt alleles.
                        if alt >= b'1' {
                            let i_alt = alt - b'1';
                            alt_counts[i_alt as usize] += 1;
                        }
                    }
                }
            }
        }
        for i in 0..alts.len() {
            let alt = alts[i];
            let alt_count = alt_counts[i];
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
            self.write.write(alt)?;
            self.write.write(b"\t")?;
            self.write.write(alt_count.to_string().as_bytes())?;
            self.write.write(b"\n")?;
        }
        Ok(())
    }

    fn get_result(&mut self) -> Result<(), Error> {
        self.write.flush()?;
        Ok(())
    }
}