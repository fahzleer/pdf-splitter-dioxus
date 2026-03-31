use dioxus::prelude::*;
use std::sync::Arc;
use crate::application::app_state::AppState;
use crate::application::split_service::SplitService;
use crate::components::chunk_preview::ChunkPreview;
use crate::components::config_panel::ConfigPanel;
use crate::domain::models::{AppScreen, JobStatus, SplitConfig};
use crate::infrastructure::command_executor::DefaultCommandExecutor;
use crate::infrastructure::file_system::NativeFileSystem;
use crate::infrastructure::pdf_reader::LopdfReader;
use crate::infrastructure::pdf_writer::LopdfWriter;

#[component]
pub fn FileLoadedScreen(mut state: Signal<AppState>) -> Element {
    let current = state.read().clone();
    let source = current.source_pdf.clone().unwrap();
    let config = current.config.clone();
    let plan = current.chunk_plan.clone().unwrap();
    let size_mb = source.file_size as f64 / 1_048_576.0;
    let source_for_config = source.clone();

    rsx! {
        div { class: "file-loaded-screen",
            div { class: "pdf-info",
                "📄 {source.path.file_name().unwrap_or_default().to_string_lossy()}    {source.total_pages} pages    {size_mb:.1} MB"
            }

            ConfigPanel {
                config: config.clone(),
                total_pages: Some(source.total_pages),
                on_config_changed: move |new_config: SplitConfig| {
                    state.write().config = new_config.clone();
                    let reader = Arc::new(LopdfReader::new());
                    let executor = Arc::new(DefaultCommandExecutor::new(
                        reader.clone(),
                        Arc::new(LopdfWriter::new()),
                        Arc::new(NativeFileSystem::new()),
                    ));
                    let service = SplitService::new(reader, executor);
                    if let Ok(new_plan) = service.plan(&source_for_config, &new_config) {
                        state.write().chunk_plan = Some(new_plan);
                    }
                }
            }

            ChunkPreview { plan: plan.clone() }

            button {
                class: "split-button",
                onclick: move |_| {
                    state.write().screen = AppScreen::Splitting;
                    state.write().job_status = JobStatus::Splitting { done: 0, total: plan.total_files };

                    let reader = Arc::new(LopdfReader::new());
                    let executor = Arc::new(DefaultCommandExecutor::new(
                        reader.clone(),
                        Arc::new(LopdfWriter::new()),
                        Arc::new(NativeFileSystem::new()),
                    ));
                    let service = SplitService::new(reader, executor);
                    let source_clone = source.clone();
                    let config_clone = config.clone();

                    spawn(async move {
                        let commands = service.build_commands(&source_clone, &config_clone).unwrap_or_default();
                        let results = service.execute(commands).await;
                        let (files, errors): (Vec<_>, Vec<_>) = results.iter()
                            .partition(|r| matches!(r, crate::domain::models::ChunkResult::Success { .. }));
                        let files: Vec<_> = files.iter().filter_map(|r| match r {
                            crate::domain::models::ChunkResult::Success { path, .. } => Some(path.clone()),
                            _ => None,
                        }).collect();
                        let errors: Vec<_> = errors.iter().filter_map(|r| match r {
                            crate::domain::models::ChunkResult::Failed { error, .. } => Some(error.clone()),
                            _ => None,
                        }).collect();

                        state.write().chunk_results = results;
                        state.write().job_status = JobStatus::Completed { files, errors };
                        state.write().screen = AppScreen::Done;
                    });
                },
                "Split!"
            }
        }
    }
}
