use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Props, Clone, PartialEq)]
pub struct FileDropZoneProps {
    pub on_file_selected: EventHandler<PathBuf>,
}

#[component]
pub fn FileDropZone(props: FileDropZoneProps) -> Element {
    rsx! {
        div {
            class: "drop-zone",
            div { class: "drop-zone-content",
                "🗂 Drop PDF file here"
            }
            button {
                onclick: move |_| {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PDF", &["pdf"])
                        .pick_file()
                    {
                        props.on_file_selected.call(path);
                    }
                },
                "Browse..."
            }
        }
    }
}
