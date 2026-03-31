use dioxus::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use crate::application::app_state::AppState;
use crate::application::image_service::ImageToPdfService;
use crate::domain::errors::SplitError;
use crate::domain::models::{AppScreen, ImageToPdfConfig, JobStatus, PageSizeOption};
use crate::infrastructure::command_executor::DefaultCommandExecutor;
use crate::infrastructure::file_system::NativeFileSystem;
use crate::infrastructure::pdf_reader::LopdfReader;
use crate::infrastructure::pdf_writer::LopdfWriter;

#[component]
pub fn ImageSetupScreen(mut state: Signal<AppState>) -> Element {
    let current = state.read().clone();
    let config = current.image_config.clone().unwrap_or(ImageToPdfConfig {
        images: vec![],
        output_path: PathBuf::from("output.pdf"),
        page_size: PageSizeOption::FitImage,
    });

    rsx! {
        div { class: "image-setup-screen",
            h2 { "Images to PDF" }

            button {
                onclick: move |_| {
                    let paths = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg"])
                        .pick_files()
                        .unwrap_or_default();
                    if !paths.is_empty() {
                        if let Ok(images) = ImageToPdfService::validate_images(&paths) {
                            let new_config = ImageToPdfConfig {
                                images,
                                ..config.clone()
                            };
                            state.write().image_config = Some(new_config);
                        }
                    }
                },
                "Select Images..."
            }

            if !config.images.is_empty() {
                div { class: "image-list",
                    p { "{config.images.len()} images selected" }
                    for img in &config.images {
                        div { class: "image-item",
                            "{img.path.file_name().unwrap_or_default().to_string_lossy()} ({img.width}x{img.height})"
                        }
                    }
                }

                div { class: "config-row",
                    label { "Page size:" }
                    input {
                        r#type: "radio",
                        name: "page-size",
                        checked: matches!(config.page_size, PageSizeOption::FitImage),
                        onchange: move |_| {
                            if let Some(ref mut c) = state.write().image_config {
                                c.page_size = PageSizeOption::FitImage;
                            }
                        }
                    }
                    span { "Fit Image" }
                    input {
                        r#type: "radio",
                        name: "page-size",
                        checked: matches!(config.page_size, PageSizeOption::A4),
                        onchange: move |_| {
                            if let Some(ref mut c) = state.write().image_config {
                                c.page_size = PageSizeOption::A4;
                            }
                        }
                    }
                    span { "A4" }
                }

                div { class: "config-row",
                    label { "Output:" }
                    span { "{config.output_path.display()}" }
                    button {
                        onclick: move |_| {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("PDF", &["pdf"])
                                .save_file()
                            {
                                if let Some(ref mut c) = state.write().image_config {
                                    c.output_path = path;
                                }
                            }
                        },
                        "Save As..."
                    }
                }

                button {
                    class: "split-button",
                    onclick: move |_| {
                        state.write().screen = AppScreen::ImageProcessing;
                        let config_clone = state.read().image_config.clone().unwrap();

                        let (tx, rx) = std::sync::mpsc::channel();

                        std::thread::spawn(move || {
                            let executor = Arc::new(DefaultCommandExecutor::new(
                                Arc::new(LopdfReader::new()),
                                Arc::new(LopdfWriter::new()),
                                Arc::new(NativeFileSystem::new()),
                            ));
                            let service = ImageToPdfService::new(executor);
                            let commands = ImageToPdfService::build_commands(&config_clone);
                            let result = service.execute(commands);
                            let _ = tx.send(result);
                        });

                        spawn(async move {
                            match rx.recv() {
                                Ok(Ok(path)) => {
                                    state.write().image_results = Some(path);
                                    state.write().screen = AppScreen::Done;
                                }
                                Ok(Err(e)) => {
                                    state.write().job_status = JobStatus::Failed { error: e };
                                    state.write().screen = AppScreen::Error {
                                        msg: "Failed to create PDF".into(),
                                    };
                                }
                                Err(e) => {
                                    state.write().job_status = JobStatus::Failed {
                                        error: SplitError::WriteError { msg: e.to_string() }
                                    };
                                    state.write().screen = AppScreen::Error {
                                        msg: "Task panicked".into(),
                                    };
                                }
                            }
                        });
                    },
                    "Combine to PDF"
                }
            }
        }
    }
}
