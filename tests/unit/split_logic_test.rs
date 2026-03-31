use pdf_splitter_dioxus::domain::errors::SplitError;
use pdf_splitter_dioxus::domain::split_logic::calculate_chunks;

#[test]
fn tc_l001_10_pages_by_2() {
    let plan = calculate_chunks(10, 2).unwrap();
    assert_eq!(plan.total_files, 5);
    assert_eq!(plan.chunks[0].start, 1);
    assert_eq!(plan.chunks[0].end, 2);
    assert_eq!(plan.chunks[4].start, 9);
    assert_eq!(plan.chunks[4].end, 10);
}

#[test]
fn tc_l002_7_pages_by_3() {
    let plan = calculate_chunks(7, 3).unwrap();
    assert_eq!(plan.total_files, 3);
    assert_eq!(plan.chunks[0].len(), 3);
    assert_eq!(plan.chunks[1].len(), 3);
    assert_eq!(plan.chunks[2].len(), 1);
}

#[test]
fn tc_l003_1_page_by_2() {
    let plan = calculate_chunks(1, 2).unwrap();
    assert_eq!(plan.total_files, 1);
    assert_eq!(plan.chunks[0].start, 1);
    assert_eq!(plan.chunks[0].end, 1);
}

#[test]
fn tc_l004_5_pages_by_1() {
    let plan = calculate_chunks(5, 1).unwrap();
    assert_eq!(plan.total_files, 5);
    for (i, chunk) in plan.chunks.iter().enumerate() {
        assert_eq!(chunk.start, i + 1);
        assert_eq!(chunk.end, i + 1);
    }
}

#[test]
fn tc_l005_ppf_equals_total() {
    let plan = calculate_chunks(10, 10).unwrap();
    assert_eq!(plan.total_files, 1);
    assert_eq!(plan.chunks[0].start, 1);
    assert_eq!(plan.chunks[0].end, 10);
}

#[test]
fn tc_l006_ppf_greater_than_total() {
    let plan = calculate_chunks(10, 20).unwrap();
    assert_eq!(plan.total_files, 1);
    assert_eq!(plan.chunks[0].end, 10);
}

#[test]
fn tc_l007_100_pages_by_7() {
    let plan = calculate_chunks(100, 7).unwrap();
    assert_eq!(plan.total_files, 15);
    assert_eq!(plan.chunks[14].start, 99);
    assert_eq!(plan.chunks[14].end, 100);
}

#[test]
fn tc_l008_total_pages_zero() {
    let result = calculate_chunks(0, 2);
    assert!(matches!(result, Err(SplitError::InvalidConfig)));
}

#[test]
fn tc_l009_ppf_zero() {
    let result = calculate_chunks(10, 0);
    assert!(matches!(result, Err(SplitError::InvalidConfig)));
}
