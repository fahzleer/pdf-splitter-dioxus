use crate::domain::commands::PdfCommand;
use crate::domain::errors::SplitError;
use crate::domain::models::{ChunkPlan, PageRange, SplitConfig, SourcePdf};
use crate::domain::naming::generate_output_path;

pub fn calculate_chunks(total_pages: usize, pages_per_file: usize) -> Result<ChunkPlan, SplitError> {
    if total_pages == 0 || pages_per_file == 0 {
        return Err(SplitError::InvalidConfig);
    }

    let chunks: Vec<PageRange> = (0..)
        .map(|i| {
            let start = i * pages_per_file + 1;
            let end = (start + pages_per_file - 1).min(total_pages);
            PageRange { start, end }
        })
        .take_while(|r| r.start <= total_pages)
        .collect();

    Ok(ChunkPlan {
        total_files: chunks.len(),
        chunks,
    })
}

pub fn validate_config(config: &SplitConfig) -> Result<(), SplitError> {
    if config.pages_per_file == 0 {
        return Err(SplitError::InvalidConfig);
    }
    Ok(())
}

pub fn plan_split(source: &SourcePdf, config: &SplitConfig) -> Result<Vec<PdfCommand>, SplitError> {
    validate_config(config)?;
    let chunk_plan = calculate_chunks(source.total_pages, config.pages_per_file)?;

    if config.dry_run {
        return Ok(vec![]);
    }

    let mut commands = vec![PdfCommand::EnsureDir {
        path: config.output_dir.clone(),
    }];

    for (i, chunk) in chunk_plan.chunks.iter().enumerate() {
        let output_path = generate_output_path(i, chunk, config)?;
        commands.push(PdfCommand::ExtractAndSave {
            doc_path: source.path.clone(),
            range: chunk.clone(),
            output_path,
            overwrite: config.overwrite,
        });
    }

    Ok(commands)
}
