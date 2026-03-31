use std::path::PathBuf;
use std::sync::Arc;
use crate::domain::commands::PdfCommand;
use crate::domain::errors::SplitError;
use crate::domain::models::{ImageSource, ImageToPdfConfig};
use crate::domain::ports::CommandExecutor;

pub struct ImageToPdfService {
    executor: Arc<dyn CommandExecutor>,
}

impl ImageToPdfService {
    pub fn new(executor: Arc<dyn CommandExecutor>) -> Self {
        ImageToPdfService { executor }
    }

    pub fn validate_images(paths: &[PathBuf]) -> Result<Vec<ImageSource>, SplitError> {
        let mut images = Vec::new();
        for path in paths {
            let img = image::open(path)
                .map_err(|e| SplitError::WriteError { msg: e.to_string() })?;
            images.push(ImageSource {
                path: path.clone(),
                width: img.width(),
                height: img.height(),
            });
        }
        Ok(images)
    }

    pub fn build_commands(config: &ImageToPdfConfig) -> Vec<PdfCommand> {
        vec![PdfCommand::CreatePdfFromImages {
            output_path: config.output_path.clone(),
            images: config.images.clone(),
            page_size: config.page_size.clone(),
        }]
    }

    pub fn execute(&self, commands: Vec<PdfCommand>) -> Result<PathBuf, SplitError> {
        let mut output = None;
        for cmd in &commands {
            self.executor.execute(cmd)?;
            if let PdfCommand::CreatePdfFromImages { output_path, .. } = cmd {
                output = Some(output_path.clone());
            }
        }
        output.ok_or_else(|| SplitError::WriteError {
            msg: "no output path generated".into(),
        })
    }
}
