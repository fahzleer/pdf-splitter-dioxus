use std::path::PathBuf;
use pdf_splitter_dioxus::domain::errors::SplitError;
use pdf_splitter_dioxus::domain::models::{NamingPattern, PageRange, SplitConfig};
use pdf_splitter_dioxus::domain::naming::generate_output_path;

fn config_with_naming(naming: NamingPattern) -> SplitConfig {
    SplitConfig {
        pages_per_file: 2,
        output_dir: PathBuf::from("./out"),
        naming,
        overwrite: false,
        dry_run: false,
    }
}

#[test]
fn tc_n001_sequential_index_0() {
    let range = PageRange::new(1, 2).unwrap();
    let path = generate_output_path(0, &range, &config_with_naming(NamingPattern::Sequential)).unwrap();
    assert_eq!(path, PathBuf::from("./out/output_1.pdf"));
}

#[test]
fn tc_n002_sequential_index_4() {
    let range = PageRange::new(1, 2).unwrap();
    let path = generate_output_path(4, &range, &config_with_naming(NamingPattern::Sequential)).unwrap();
    assert_eq!(path, PathBuf::from("./out/output_5.pdf"));
}

#[test]
fn tc_n003_with_page_range() {
    let range = PageRange::new(1, 3).unwrap();
    let path = generate_output_path(0, &range, &config_with_naming(NamingPattern::WithPageRange)).unwrap();
    assert_eq!(path, PathBuf::from("./out/pages_1-3.pdf"));
}

#[test]
fn tc_n004_custom_prefix() {
    let range = PageRange::new(1, 2).unwrap();
    let path = generate_output_path(2, &range, &config_with_naming(NamingPattern::Custom { prefix: "scan".into() })).unwrap();
    assert_eq!(path, PathBuf::from("./out/scan_3.pdf"));
}

#[test]
fn tc_n005_custom_prefix_with_space() {
    let range = PageRange::new(1, 2).unwrap();
    let path = generate_output_path(0, &range, &config_with_naming(NamingPattern::Custom { prefix: "my doc".into() })).unwrap();
    assert_eq!(path, PathBuf::from("./out/my doc_1.pdf"));
}

#[test]
fn tc_n006_path_traversal_rejected() {
    let range = PageRange::new(1, 2).unwrap();
    let result = generate_output_path(0, &range, &config_with_naming(NamingPattern::Custom { prefix: "../../evil".into() }));
    assert!(matches!(result, Err(SplitError::InvalidConfig)));
}
