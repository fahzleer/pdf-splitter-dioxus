use std::path::PathBuf;
use crate::domain::errors::SplitError;
use crate::domain::models::{NamingPattern, PageRange, SplitConfig};

pub fn generate_output_path(index: usize, range: &PageRange, config: &SplitConfig) -> Result<PathBuf, SplitError> {
    let filename = match &config.naming {
        NamingPattern::Sequential => format!("output_{}.pdf", index + 1),
        NamingPattern::WithPageRange => format!("pages_{}-{}.pdf", range.start, range.end),
        NamingPattern::Custom { prefix } => {
            if prefix.contains("..") || prefix.contains('/') || prefix.contains('\\') {
                return Err(SplitError::InvalidConfig);
            }
            format!("{}_{}.pdf", prefix, index + 1)
        }
    };
    Ok(config.output_dir.join(filename))
}
