use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use crate::application::app_state::AppState;
use crate::application::split_service::SplitService;
use crate::components::config_panel::ConfigPanel;
use crate::components::file_drop_zone::FileDropZone;
use crate::domain::models::{AppMode, AppScreen, JobStatus, SplitConfig};
use crate::infrastructure::command_executor::DefaultCommandExecutor;
use crate::infrastructure::file_system::NativeFileSystem;
use crate::infrastructure::pdf_reader::LopdfReader;
use crate::infrastructure::pdf_writer::LopdfWriter;

#[component]
pub fn WelcomeScreen(mut state: Signal<AppState>) -> Element {
    let config = state.read().config.clone();
    let mode = state.read().app_mode.clone();

    rsx! {
        div { class: "welcome-screen",
            h1 { "PDF Tools" }

            div { class: "mode-selector",
                button {
                    class: if matches!(mode, AppMode::SplitPdf) { "mode-btn active" } else { "mode-btn" },
                    onclick: move |_| { state.write().app_mode = AppMode::SplitPdf; },
                    "Split PDF"
                }
                button {
                    class: if matches!(mode, AppMode::ImagesToPdf) { "mode-btn active" } else { "mode-btn" },
                    onclick: move |_| { state.write().app_mode = AppMode::ImagesToPdf; },
                    "Images to PDF"
                }
            }

            match mode {
                AppMode::SplitPdf => rsx! {
                    p { "Configure settings below, then select a PDF file to split." }
                    ConfigPanel {
                        config: config.clone(),
                        total_pages: None,
                        on_config_changed: move |new_config: SplitConfig| {
                            state.write().config = new_config;
                        }
                    }
                    FileDropZone {
                        on_file_selected: move |path: PathBuf| {
                            let reader = Arc::new(LopdfReader::new());
                            let executor = Arc::new(DefaultCommandExecutor::new(
                                reader.clone(),
                                Arc::new(LopdfWriter::new()),
                                Arc::new(NativeFileSystem::new()),
                            ));
                            let service = SplitService::new(reader, executor);
                            match service.validate(&path) {
                                Ok(source) => {
                                    let plan = service.plan(&source, &state.read().config);
                                    state.write().source_pdf = Some(source);
                                    match plan {
                                        Ok(p) => {
                                            state.write().chunk_plan = Some(p);
                                            state.write().screen = AppScreen::FileLoaded;
                                        }
                                        Err(e) => {
                                            state.write().job_status = JobStatus::Failed { error: e };
                                            state.write().screen = AppScreen::Error {
                                                msg: "Failed to plan split".into(),
                                            };
                                        }
                                    }
                                }
                                Err(e) => {
                                    state.write().job_status = JobStatus::Failed { error: e };
                                    state.write().screen = AppScreen::Error {
                                        msg: "Invalid PDF".into(),
                                    };
                                }
                            }
                        }
                    }
                },
                AppMode::ImagesToPdf => rsx! {
                    p { "Select multiple images to combine into a single PDF." }
                    button {
                        class: "split-button",
                        onclick: move |_| { state.write().screen = AppScreen::ImageSetup; },
                        "Get Started"
                    }
                },
            }
        }
    }
}
