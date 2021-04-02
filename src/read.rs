use vcf::{VCFRecord, VCFReader};
use crate::error;
use flate2::read::MultiGzDecoder;
use std::io::BufReader;
use std::io;
use std::fs::File;

pub trait VcfRecordInspector<R> {
    fn reset(&mut self) -> ();
    fn inspect_record(&mut self, record: &VCFRecord) -> ();
    fn get_result(&self) -> R;
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
    fn reset(&mut self) -> () {
        self.n_records = 0;
    }

    fn inspect_record(&mut self, _record: &VCFRecord) -> () {
        self.n_records += 1;
    }

    fn get_result(&self) -> u32 {
        self.n_records
    }
}

pub fn apply_record_inspector<B: io::BufRead, R, I: VcfRecordInspector<R>>
(reader: &mut VCFReader<B>, inspector: &mut I) -> Result<R, error::Error> {
    let mut record = reader.empty_record();
    loop {
        let has_record = reader.next_record(&mut record)?;
        if has_record {
            inspector.inspect_record(&record);
        } else {
            break Ok(inspector.get_result());
        }
    }
}

