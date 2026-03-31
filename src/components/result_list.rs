use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Props, Clone, PartialEq)]
pub struct ResultListProps {
    pub files: Vec<PathBuf>,
}

#[component]
pub fn ResultList(props: ResultListProps) -> Element {
    rsx! {
        div { class: "result-list",
            for file in &props.files {
                div { class: "result-item",
                    span { "{file.file_name().unwrap_or_default().to_string_lossy()}" }
                }
            }
        }
    }
}
