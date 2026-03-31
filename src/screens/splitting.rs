use dioxus::prelude::*;
use crate::application::app_state::AppState;
use crate::components::split_progress::SplitProgress;

#[component]
pub fn SplittingScreen(state: Signal<AppState>) -> Element {
    let current = state.read().clone();
    let (done, total) = match &current.job_status {
        crate::domain::models::JobStatus::Splitting { done, total } => (*done, *total),
        _ => (0, 0),
    };

    let current_file = current.chunk_results.last().and_then(|r| match r {
        crate::domain::models::ChunkResult::Success { path, .. } => {
            Some(path.file_name().unwrap_or_default().to_string_lossy().to_string())
        }
        _ => None,
    });

    rsx! {
        div { class: "splitting-screen",
            h2 { "Splitting..." }
            SplitProgress { done, total, current_file }
            div { class: "chunk-status-list",
                for result in &current.chunk_results {
                    match result {
                        crate::domain::models::ChunkResult::Success { path, .. } => {
                            rsx! {
                                div { class: "chunk-status success",
                                    "✓ {path.file_name().unwrap_or_default().to_string_lossy()}"
                                }
                            }
                        }
                        crate::domain::models::ChunkResult::Failed { index, error } => {
                            rsx! {
                                div { class: "chunk-status error",
                                    "✗ Chunk {index}: {error.to_string()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
