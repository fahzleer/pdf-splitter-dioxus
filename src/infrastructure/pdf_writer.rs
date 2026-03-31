use std::path::Path;
use crate::domain::errors::SplitError;
use crate::domain::models::PageRange;
use crate::domain::ports::PdfDocument;
use crate::domain::ports::PdfWriter;

pub struct LopdfWriter;

impl LopdfWriter {
    pub fn new() -> Self {
        LopdfWriter
    }
}

impl PdfWriter for LopdfWriter {
    fn extract_pages(&self, doc: &PdfDocument, range: &PageRange) -> Result<Vec<u8>, SplitError> {
        let mut new_doc = doc.inner.clone();
        let pages = new_doc.get_pages();
        let total_pages = pages.len() as u32;
        let keep: Vec<u32> = (range.start as u32..=range.end as u32).collect();

        let remove: Vec<u32> = (1..=total_pages)
            .filter(|p| !keep.contains(p))
            .collect();

        new_doc.delete_pages(&remove);
        new_doc.prune_objects();

        let mut buf = Vec::new();
        new_doc
            .save_to(&mut buf)
            .map_err(|e| SplitError::WriteError { msg: e.to_string() })?;

        Ok(buf)
    }

    fn save(&self, data: &[u8], path: &Path) -> Result<(), SplitError> {
        std::fs::write(path, data).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => SplitError::PermissionDenied {
                path: path.to_path_buf(),
            },
            std::io::ErrorKind::WriteZero => SplitError::DiskFull,
            _ => SplitError::WriteError { msg: e.to_string() },
        })
    }
}
