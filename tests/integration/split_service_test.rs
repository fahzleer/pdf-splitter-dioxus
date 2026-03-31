use std::sync::Arc;
use tempfile::TempDir;
use pdf_splitter_dioxus::application::split_service::SplitService;
use pdf_splitter_dioxus::domain::models::{ChunkResult, NamingPattern, SplitConfig};
use pdf_splitter_dioxus::domain::ports::PdfReader;
use pdf_splitter_dioxus::infrastructure::command_executor::DefaultCommandExecutor;
use pdf_splitter_dioxus::infrastructure::file_system::NativeFileSystem;
use pdf_splitter_dioxus::infrastructure::pdf_reader::LopdfReader;
use pdf_splitter_dioxus::infrastructure::pdf_writer::LopdfWriter;
use crate::integration::helpers::create_test_pdf;

fn make_service() -> SplitService {
    let reader = Arc::new(LopdfReader::new());
    let executor = Arc::new(DefaultCommandExecutor::new(
        reader.clone(),
        Arc::new(LopdfWriter::new()),
        Arc::new(NativeFileSystem::new()),
    ));
    SplitService::new(reader, executor)
}

#[tokio::test]
async fn tc_e001_end_to_end_10_pages_by_2() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 10);

    let service = make_service();
    let source = service.validate(&input).unwrap();
    let config = SplitConfig {
        pages_per_file: 2,
        output_dir: output_dir.clone(),
        naming: NamingPattern::Sequential,
        overwrite: false,
        dry_run: false,
    };
    let plan = service.plan(&source, &config).unwrap();
    assert_eq!(plan.total_files, 5);

    let commands = service.build_commands(&source, &config).unwrap();
    let results = service.execute(commands).await;
    assert_eq!(results.len(), 5);

    let successes: Vec<_> = results.iter().filter(|r| matches!(r, ChunkResult::Success { .. })).collect();
    assert_eq!(successes.len(), 5);
    assert!(output_dir.join("output_1.pdf").exists());
    assert!(output_dir.join("output_5.pdf").exists());
}

#[tokio::test]
async fn tc_e002_7_pages_by_3() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 7);

    let service = make_service();
    let source = service.validate(&input).unwrap();
    let config = SplitConfig {
        pages_per_file: 3,
        output_dir: output_dir.clone(),
        naming: NamingPattern::Sequential,
        overwrite: false,
        dry_run: false,
    };
    let commands = service.build_commands(&source, &config).unwrap();
    let results = service.execute(commands).await;

    assert_eq!(results.len(), 3);
    match &results[2] {
        ChunkResult::Success { path, .. } => {
            let out_doc = LopdfReader::new().load(path).unwrap();
            assert_eq!(LopdfReader::new().count_pages(&out_doc), 1);
        }
        _ => panic!("expected success"),
    }
}

#[tokio::test]
async fn tc_e003_single_page_pdf() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 1);

    let service = make_service();
    let source = service.validate(&input).unwrap();
    let config = SplitConfig {
        pages_per_file: 2,
        output_dir: output_dir.clone(),
        naming: NamingPattern::Sequential,
        overwrite: false,
        dry_run: false,
    };
    let commands = service.build_commands(&source, &config).unwrap();
    let results = service.execute(commands).await;

    assert_eq!(results.len(), 1);
    assert!(matches!(results[0], ChunkResult::Success { .. }));
}

#[tokio::test]
async fn tc_e005_overwrite_false() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 10);
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::write(output_dir.join("output_1.pdf"), b"existing").unwrap();

    let service = make_service();
    let source = service.validate(&input).unwrap();
    let config = SplitConfig {
        pages_per_file: 2,
        output_dir: output_dir.clone(),
        naming: NamingPattern::Sequential,
        overwrite: false,
        dry_run: false,
    };
    let commands = service.build_commands(&source, &config).unwrap();
    let results = service.execute(commands).await;

    assert!(matches!(results[0], ChunkResult::Failed { .. }));
}

#[tokio::test]
async fn tc_e006_overwrite_true() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 10);
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::write(output_dir.join("output_1.pdf"), b"existing").unwrap();

    let service = make_service();
    let source = service.validate(&input).unwrap();
    let config = SplitConfig {
        pages_per_file: 2,
        output_dir: output_dir.clone(),
        naming: NamingPattern::Sequential,
        overwrite: true,
        dry_run: false,
    };
    let commands = service.build_commands(&source, &config).unwrap();
    let results = service.execute(commands).await;

    assert!(matches!(results[0], ChunkResult::Success { .. }));
}

#[tokio::test]
async fn tc_e007_dry_run() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 10);

    let service = make_service();
    let source = service.validate(&input).unwrap();
    let config = SplitConfig {
        pages_per_file: 2,
        output_dir: output_dir.clone(),
        naming: NamingPattern::Sequential,
        overwrite: false,
        dry_run: true,
    };
    let commands = service.build_commands(&source, &config).unwrap();
    let results = service.execute(commands).await;

    assert!(results.is_empty());
    assert!(!output_dir.join("output_1.pdf").exists());
}
