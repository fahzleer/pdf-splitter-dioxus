use pdf_splitter_dioxus::domain::errors::SplitError;
use pdf_splitter_dioxus::domain::models::{JobStatus, NamingPattern, PageRange, SplitConfig};
use pdf_splitter_dioxus::domain::split_logic::validate_config;

#[test]
fn tc_m001_create_page_range() {
    let range = PageRange::new(1, 2).unwrap();
    assert_eq!(range.start, 1);
    assert_eq!(range.end, 2);
}

#[test]
fn tc_m002_page_range_len() {
    let range = PageRange::new(3, 7).unwrap();
    assert_eq!(range.len(), 5);
}

#[test]
fn tc_m003_invalid_start_greater_than_end() {
    let result = PageRange::new(5, 2);
    assert!(matches!(result, Err(SplitError::InvalidConfig)));
}

#[test]
fn tc_m004_invalid_start_zero() {
    let result = PageRange::new(0, 2);
    assert!(matches!(result, Err(SplitError::InvalidConfig)));
}

#[test]
fn tc_m005_single_page() {
    let range = PageRange::new(3, 3).unwrap();
    assert_eq!(range.len(), 1);
}

#[test]
fn tc_c001_split_config_default() {
    let config = SplitConfig::default();
    assert_eq!(config.pages_per_file, 2);
    assert!(matches!(config.naming, NamingPattern::Sequential));
    assert!(!config.overwrite);
    assert!(!config.dry_run);
}

#[test]
fn tc_c002_pages_per_file_zero() {
    let config = SplitConfig {
        pages_per_file: 0,
        ..SplitConfig::default()
    };
    let result = validate_config(&config);
    assert!(matches!(result, Err(SplitError::InvalidConfig)));
}

#[test]
fn tc_c003_immutable_clone() {
    let config = SplitConfig::default();
    let new_config = SplitConfig {
        pages_per_file: 5,
        ..config.clone()
    };
    assert_eq!(config.pages_per_file, 2);
    assert_eq!(new_config.pages_per_file, 5);
}

#[test]
fn tc_j001_pattern_match_exhaustive() {
    let status = JobStatus::Splitting { done: 3, total: 5 };
    let progress = match status {
        JobStatus::Idle => 0,
        JobStatus::Validating => 0,
        JobStatus::Ready { .. } => 0,
        JobStatus::Splitting { done, total } => done * 100 / total,
        JobStatus::Completed { .. } => 100,
        JobStatus::Failed { .. } => 0,
    };
    assert_eq!(progress, 60);
}

#[test]
fn tc_j002_splitting_progress() {
    let status = JobStatus::Splitting { done: 3, total: 5 };
    match status {
        JobStatus::Splitting { done, total } => {
            assert_eq!(done, 3);
            assert_eq!(total, 5);
            assert_eq!(done * 100 / total, 60);
        }
        _ => panic!("expected Splitting status"),
    }
}
