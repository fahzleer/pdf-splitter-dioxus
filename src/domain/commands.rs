use std::path::PathBuf;
use crate::domain::models::{ImageSource, PageRange, PageSizeOption};

#[derive(Debug, Clone)]
pub enum PdfCommand {
    EnsureDir { path: PathBuf },
    ExtractAndSave {
        doc_path: PathBuf,
        range: PageRange,
        output_path: PathBuf,
        overwrite: bool,
    },
    CreatePdfFromImages {
        output_path: PathBuf,
        images: Vec<ImageSource>,
        page_size: PageSizeOption,
    },
    #[allow(dead_code)]
    Log { level: LogLevel, message: String },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PdfDocRef(pub String);

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}
