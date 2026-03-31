use std::path::PathBuf;
use crate::domain::errors::SplitError;

#[derive(Debug, Clone, PartialEq)]
pub enum NamingPattern {
    Sequential,
    WithPageRange,
    Custom { prefix: String },
}

impl Default for NamingPattern {
    fn default() -> Self {
        NamingPattern::Sequential
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Idle,
    #[allow(dead_code)]
    Validating,
    #[allow(dead_code)]
    Ready { total_pages: usize },
    Splitting { done: usize, total: usize },
    Completed { files: Vec<PathBuf>, errors: Vec<SplitError> },
    Failed { error: SplitError },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Welcome,
    FileLoaded,
    Splitting,
    Done,
    Error { msg: String },
    ImageSetup,
    ImageProcessing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    SplitPdf,
    ImagesToPdf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageSizeOption {
    FitImage,
    A4,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageRange {
    pub start: usize,
    pub end: usize,
}

impl PageRange {
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize) -> Result<Self, SplitError> {
        if start == 0 || start > end {
            return Err(SplitError::InvalidConfig);
        }
        Ok(PageRange { start, end })
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SplitConfig {
    pub pages_per_file: usize,
    pub output_dir: PathBuf,
    pub naming: NamingPattern,
    pub overwrite: bool,
    pub dry_run: bool,
}

impl Default for SplitConfig {
    fn default() -> Self {
        SplitConfig {
            pages_per_file: 2,
            output_dir: PathBuf::from("."),
            naming: NamingPattern::default(),
            overwrite: false,
            dry_run: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourcePdf {
    pub path: PathBuf,
    pub total_pages: usize,
    pub file_size: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkPlan {
    pub chunks: Vec<PageRange>,
    pub total_files: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkResult {
    Success { index: usize, path: PathBuf, page_range: PageRange },
    Failed { index: usize, error: SplitError },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageSource {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageToPdfConfig {
    pub images: Vec<ImageSource>,
    pub output_path: PathBuf,
    pub page_size: PageSizeOption,
}
