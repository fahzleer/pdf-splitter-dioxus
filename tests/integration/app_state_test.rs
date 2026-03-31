use pdf_splitter_dioxus::application::app_state::AppState;
use pdf_splitter_dioxus::domain::models::{AppScreen, JobStatus, SourcePdf};

#[test]
fn tc_s001_initial_state() {
    let state = AppState::new();
    assert_eq!(state.screen, AppScreen::Welcome);
    assert!(state.source_pdf.is_none());
    assert!(matches!(state.job_status, JobStatus::Idle));
}

#[test]
fn tc_s002_with_file_does_not_mutate() {
    let state = AppState::new();
    let pdf = SourcePdf {
        path: std::path::PathBuf::from("test.pdf"),
        total_pages: 10,
        file_size: 1024,
    };
    let new_state = state.clone().with_file(pdf.clone());
    assert!(state.source_pdf.is_none());
    assert_eq!(new_state.source_pdf, Some(pdf));
}

#[test]
fn tc_s003_with_config_does_not_mutate() {
    let state = AppState::new();
    let config = pdf_splitter_dioxus::domain::models::SplitConfig {
        pages_per_file: 5,
        ..state.config.clone()
    };
    let new_state = state.clone().with_config(config.clone());
    assert_eq!(state.config.pages_per_file, 2);
    assert_eq!(new_state.config.pages_per_file, 5);
}

#[test]
fn tc_s004_with_status_splitting_changes_screen() {
    let state = AppState::new();
    let new_state = state.with_status(JobStatus::Splitting { done: 0, total: 5 });
    assert_eq!(new_state.screen, AppScreen::Splitting);
}

#[test]
fn tc_s005_with_status_completed_changes_screen() {
    let state = AppState::new();
    let new_state = state.with_status(JobStatus::Completed { files: vec![], errors: vec![] });
    assert_eq!(new_state.screen, AppScreen::Done);
}
