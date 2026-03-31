use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum SplitError {
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    #[error("Not a PDF file: {path}")]
    NotAPdf { path: PathBuf },
    #[error("PDF is corrupted: {path}")]
    CorruptedPdf { path: PathBuf },
    #[error("PDF has no pages: {path}")]
    EmptyPdf { path: PathBuf },
    #[error("PDF is password protected: {path}")]
    EncryptedPdf { path: PathBuf },
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
    #[error("Invalid config: pages_per_file must be > 0")]
    InvalidConfig,
    #[error("Disk full")]
    DiskFull,
    #[error("Write error: {msg}")]
    WriteError { msg: String },
}
