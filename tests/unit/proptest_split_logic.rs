use proptest::prelude::*;
use pdf_splitter_dioxus::domain::split_logic::calculate_chunks;

proptest! {
    #[test]
    fn tc_l010_sum_of_pages_equals_total(total in 1usize..=1000, ppf in 1usize..=100) {
        let plan = calculate_chunks(total, ppf).unwrap();
        let sum: usize = plan.chunks.iter().map(|c| c.len()).sum();
        prop_assert_eq!(sum, total);
    }

    #[test]
    fn tc_l011_chunk_count_equals_ceil_div(total in 1usize..=1000, ppf in 1usize..=100) {
        let plan = calculate_chunks(total, ppf).unwrap();
        let expected = (total + ppf - 1) / ppf;
        prop_assert_eq!(plan.total_files, expected);
    }

    #[test]
    fn tc_l012_each_chunk_has_valid_page_count(total in 1usize..=1000, ppf in 1usize..=100) {
        let plan = calculate_chunks(total, ppf).unwrap();
        for chunk in &plan.chunks {
            prop_assert!(chunk.len() >= 1 && chunk.len() <= ppf);
        }
    }

    #[test]
    fn tc_l013_chunks_are_continuous(total in 1usize..=1000, ppf in 1usize..=100) {
        let plan = calculate_chunks(total, ppf).unwrap();
        prop_assert_eq!(plan.chunks[0].start, 1);
        prop_assert_eq!(plan.chunks.last().unwrap().end, total);
        for i in 0..plan.chunks.len().saturating_sub(1) {
            prop_assert_eq!(plan.chunks[i].end + 1, plan.chunks[i + 1].start);
        }
    }
}
