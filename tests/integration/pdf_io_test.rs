use std::path::PathBuf;
use tempfile::TempDir;
use pdf_splitter_dioxus::domain::errors::SplitError;
use pdf_splitter_dioxus::domain::models::PageRange;
use pdf_splitter_dioxus::domain::ports::{FileSystem, PdfReader, PdfWriter};
use pdf_splitter_dioxus::infrastructure::file_system::NativeFileSystem;
use pdf_splitter_dioxus::infrastructure::pdf_reader::{validate_pdf, LopdfReader};
use pdf_splitter_dioxus::infrastructure::pdf_writer::LopdfWriter;
use crate::integration::helpers::create_test_pdf;

#[test]
fn tc_v001_normal_pdf_10_pages() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("test.pdf");
    create_test_pdf(&path, 10);

    let reader = LopdfReader::new();
    let result = validate_pdf(&path, &reader);
    assert!(result.is_ok());
    let (pages, size) = result.unwrap();
    assert_eq!(pages, 10);
    assert!(size > 0);
}

#[test]
fn tc_v002_file_not_found() {
    let reader = LopdfReader::new();
    let result = validate_pdf(PathBuf::from("nonexistent.pdf").as_path(), &reader);
    assert!(matches!(result, Err(SplitError::FileNotFound { .. })));
}

#[test]
fn tc_v003_not_a_pdf() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("fake.pdf");
    std::fs::write(&path, b"this is not a pdf").unwrap();

    let reader = LopdfReader::new();
    let result = validate_pdf(&path, &reader);
    assert!(matches!(result, Err(SplitError::NotAPdf { .. })));
}

#[test]
fn tc_v004_corrupted_pdf() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("corrupted.pdf");
    std::fs::write(&path, b"%PDF-1.4\ninvalid data here").unwrap();

    let reader = LopdfReader::new();
    let result = validate_pdf(&path, &reader);
    assert!(matches!(result, Err(SplitError::CorruptedPdf { .. })));
}

#[test]
fn tc_v005_empty_pdf() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("empty.pdf");
    create_test_pdf(&path, 0);

    let reader = LopdfReader::new();
    let result = validate_pdf(&path, &reader);
    assert!(matches!(result, Err(SplitError::EmptyPdf { .. })));
}

#[test]
fn tc_e001_split_10_pages_by_2() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.pdf");
    let output_dir = tmp.path().join("output");
    create_test_pdf(&input, 10);

    let reader = LopdfReader::new();
    let writer = LopdfWriter::new();
    let fs = NativeFileSystem::new();

    let doc = reader.load(&input).unwrap();
    let range = PageRange::new(1, 2).unwrap();
    let data = writer.extract_pages(&doc, &range).unwrap();

    let out_path = output_dir.join("output_1.pdf");
    fs.ensure_dir(&output_dir).unwrap();
    writer.save(&data, &out_path).unwrap();

    assert!(fs.file_exists(&out_path));

    let out_doc = reader.load(&out_path).unwrap();
    assert_eq!(reader.count_pages(&out_doc), 2);
}

#[test]
fn tc_e004_create_output_dir() {
    let tmp = TempDir::new().unwrap();
    let new_dir = tmp.path().join("a").join("b").join("c");
    let fs = NativeFileSystem::new();
    assert!(!fs.file_exists(&new_dir));
    fs.ensure_dir(&new_dir).unwrap();
    assert!(fs.file_exists(&new_dir));
}
