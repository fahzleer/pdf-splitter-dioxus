use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SplitProgressProps {
    pub done: usize,
    pub total: usize,
    pub current_file: Option<String>,
}

#[component]
pub fn SplitProgress(props: SplitProgressProps) -> Element {
    let percent = if props.total > 0 {
        props.done * 100 / props.total
    } else {
        0
    };

    rsx! {
        div { class: "split-progress",
            div { class: "progress-bar-container",
                div {
                    class: "progress-bar",
                    style: "width: {percent}%"
                }
            }
            p { "{props.done} / {props.total} files" }
            if let Some(file) = &props.current_file {
                p { class: "current-file", "Creating: {file}" }
            }
        }
    }
}
