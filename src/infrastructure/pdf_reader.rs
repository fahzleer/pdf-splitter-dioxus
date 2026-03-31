use std::fs;
use std::path::Path;
use crate::domain::errors::SplitError;
use crate::domain::ports::{PdfDocument, PdfReader};

pub struct LopdfReader;

impl LopdfReader {
    pub fn new() -> Self {
        LopdfReader
    }
}

impl PdfReader for LopdfReader {
    fn load(&self, path: &Path) -> Result<PdfDocument, SplitError> {
        if !path.exists() {
            return Err(SplitError::FileNotFound { path: path.to_path_buf() });
        }

        let metadata = fs::metadata(path).map_err(|_| SplitError::PermissionDenied {
            path: path.to_path_buf(),
        })?;

        if metadata.permissions().readonly() && false {
            // readonly is not the same as permission denied on all platforms
        }

        let bytes = fs::read(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => SplitError::PermissionDenied {
                path: path.to_path_buf(),
            },
            _ => SplitError::CorruptedPdf { path: path.to_path_buf() },
        })?;

        if bytes.len() < 4 || &bytes[0..4] != b"%PDF" {
            return Err(SplitError::NotAPdf { path: path.to_path_buf() });
        }

        let doc = lopdf::Document::load(path).map_err(|_| SplitError::CorruptedPdf {
            path: path.to_path_buf(),
        })?;

        Ok(PdfDocument { inner: doc })
    }

    fn count_pages(&self, doc: &PdfDocument) -> usize {
        doc.inner.get_pages().len()
    }

    fn is_encrypted(&self, doc: &PdfDocument) -> bool {
        doc.inner.is_encrypted()
    }
}

pub fn validate_pdf(path: &Path, reader: &dyn PdfReader) -> Result<(usize, u64), SplitError> {
    let doc = reader.load(path)?;
    let pages = reader.count_pages(&doc);
    let size = fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| SplitError::WriteError { msg: e.to_string() })?;

    if pages == 0 {
        return Err(SplitError::EmptyPdf { path: path.to_path_buf() });
    }

    if reader.is_encrypted(&doc) {
        return Err(SplitError::EncryptedPdf { path: path.to_path_buf() });
    }

    Ok((pages, size))
}
