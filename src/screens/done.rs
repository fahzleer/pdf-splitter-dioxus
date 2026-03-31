use dioxus::prelude::*;
use crate::application::app_state::AppState;
use crate::components::result_list::ResultList;
use crate::domain::models::AppMode;
use crate::domain::ports::FileSystem;
use crate::infrastructure::file_system::NativeFileSystem;

#[component]
pub fn DoneScreen(mut state: Signal<AppState>) -> Element {
    let current = state.read().clone();
    let output_path = match current.app_mode {
        AppMode::SplitPdf => current.config.output_dir.clone(),
        AppMode::ImagesToPdf => current.image_results.clone().unwrap_or_default(),
    };

    let files = match &current.job_status {
        crate::domain::models::JobStatus::Completed { files, .. } => files.clone(),
        _ => vec![],
    };

    rsx! {
        div { class: "done-screen",
            match current.app_mode {
                AppMode::SplitPdf => rsx! {
                    h2 { "Done! Created {files.len()} files" }
                    ResultList { files: files.clone() }
                },
                AppMode::ImagesToPdf => rsx! {
                    h2 { "Done! PDF created successfully" }
                    p { "Saved to: {output_path.display()}" }
                },
            }
            div { class: "done-actions",
                button {
                    onclick: move |_| {
                        NativeFileSystem::new().open_in_explorer(&output_path);
                    },
                    "Open Output Folder"
                }
                button {
                    onclick: move |_| {
                        state.set(AppState::new());
                    },
                    "Start Over"
                }
            }
        }
    }
}
