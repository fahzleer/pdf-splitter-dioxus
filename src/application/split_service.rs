use std::path::Path;
use std::sync::Arc;
use crate::domain::commands::PdfCommand;
use crate::domain::errors::SplitError;
use crate::domain::models::{ChunkPlan, ChunkResult, PageRange, SourcePdf, SplitConfig};
use crate::domain::ports::{CommandExecutor, PdfReader};
use crate::domain::split_logic::{calculate_chunks, plan_split};
use crate::infrastructure::pdf_reader::validate_pdf;

pub struct SplitService {
    reader: Arc<dyn PdfReader>,
    executor: Arc<dyn CommandExecutor>,
}

impl SplitService {
    pub fn new(reader: Arc<dyn PdfReader>, executor: Arc<dyn CommandExecutor>) -> Self {
        SplitService { reader, executor }
    }

    pub fn validate(&self, path: &Path) -> Result<SourcePdf, SplitError> {
        let (pages, size) = validate_pdf(path, self.reader.as_ref())?;
        Ok(SourcePdf {
            path: path.to_path_buf(),
            total_pages: pages,
            file_size: size,
        })
    }

    pub fn plan(&self, source: &SourcePdf, config: &SplitConfig) -> Result<ChunkPlan, SplitError> {
        calculate_chunks(source.total_pages, config.pages_per_file)
    }

    pub fn build_commands(&self, source: &SourcePdf, config: &SplitConfig) -> Result<Vec<PdfCommand>, SplitError> {
        plan_split(source, config)
    }

    pub async fn execute(&self, commands: Vec<PdfCommand>) -> Vec<ChunkResult> {
        let mut results = Vec::new();
        let mut extract_index = 0;

        for command in commands {
            match &command {
                PdfCommand::ExtractAndSave { output_path, range, .. } => {
                    let result = match self.executor.execute(&command) {
                        Ok(()) => ChunkResult::Success {
                            index: extract_index,
                            path: output_path.clone(),
                            page_range: PageRange {
                                start: range.start,
                                end: range.end,
                            },
                        },
                        Err(e) => ChunkResult::Failed {
                            index: extract_index,
                            error: e,
                        },
                    };
                    results.push(result);
                    extract_index += 1;
                }
                _ => {
                    if let Err(e) = self.executor.execute(&command) {
                        results.push(ChunkResult::Failed {
                            index: extract_index,
                            error: e,
                        });
                    }
                }
            }
        }

        results
    }
}
