use std::path::Path;
use crate::domain::commands::PdfCommand;
use crate::domain::errors::SplitError;
use crate::domain::models::PageRange;

pub struct PdfDocument {
    pub(crate) inner: lopdf::Document,
}

pub trait PdfReader: Send + Sync {
    fn load(&self, path: &Path) -> Result<PdfDocument, SplitError>;
    fn count_pages(&self, doc: &PdfDocument) -> usize;
    fn is_encrypted(&self, doc: &PdfDocument) -> bool;
}

pub trait PdfWriter: Send + Sync {
    fn extract_pages(&self, doc: &PdfDocument, range: &PageRange) -> Result<Vec<u8>, SplitError>;
    fn save(&self, data: &[u8], path: &Path) -> Result<(), SplitError>;
}

pub trait FileSystem: Send + Sync {
    fn ensure_dir(&self, path: &Path) -> Result<(), SplitError>;
    fn file_exists(&self, path: &Path) -> bool;
    fn open_in_explorer(&self, path: &Path);
}

pub trait CommandExecutor: Send + Sync {
    fn execute(&self, command: &PdfCommand) -> Result<(), SplitError>;
}
