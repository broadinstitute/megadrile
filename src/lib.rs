use vcf::VCFRecord;

pub mod config;
pub mod error;

pub trait VcfRecordInspector<R> {
    fn reset(&mut self) -> ();
    fn inspect_record(&mut self, record: &VCFRecord) -> ();
    fn get_result(&self) -> R;
}
