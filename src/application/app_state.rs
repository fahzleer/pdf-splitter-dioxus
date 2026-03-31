use crate::domain::models::{AppMode, AppScreen, ChunkPlan, ChunkResult, ImageToPdfConfig, JobStatus, SourcePdf, SplitConfig};

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub screen: AppScreen,
    pub app_mode: AppMode,
    pub source_pdf: Option<SourcePdf>,
    pub config: SplitConfig,
    pub chunk_plan: Option<ChunkPlan>,
    pub job_status: JobStatus,
    pub chunk_results: Vec<ChunkResult>,
    pub image_config: Option<ImageToPdfConfig>,
    pub image_results: Option<std::path::PathBuf>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            screen: AppScreen::Welcome,
            app_mode: AppMode::SplitPdf,
            source_pdf: None,
            config: SplitConfig::default(),
            chunk_plan: None,
            job_status: JobStatus::Idle,
            chunk_results: vec![],
            image_config: None,
            image_results: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_file(self, pdf: SourcePdf) -> Self {
        Self {
            source_pdf: Some(pdf),
            screen: AppScreen::FileLoaded,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_config(self, config: SplitConfig) -> Self {
        Self {
            config,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn with_status(self, status: JobStatus) -> Self {
        let screen = match &status {
            JobStatus::Splitting { .. } => AppScreen::Splitting,
            JobStatus::Completed { .. } => AppScreen::Done,
            JobStatus::Failed { .. } => AppScreen::Error {
                msg: "Split failed".into(),
            },
            _ => self.screen.clone(),
        };
        Self {
            job_status: status,
            screen,
            ..self
        }
    }
}
