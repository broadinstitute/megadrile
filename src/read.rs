use vcf::{VCFRecord, VCFReader};
use crate::error;
use flate2::read::MultiGzDecoder;
use std::io::{BufReader, Write};
use std::io;
use std::fs::File;

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

struct VariantListWriter<W: Write> {
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
        }
        Ok(())
    }

    fn get_result(&mut self) -> Result<(), error::Error> {
        self.write.flush()?;
        Ok(())
    }
}