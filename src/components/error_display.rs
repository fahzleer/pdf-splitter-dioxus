use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorDisplayProps {
    pub message: String,
}

#[component]
pub fn ErrorDisplay(props: ErrorDisplayProps) -> Element {
    rsx! {
        div { class: "error-display",
            h2 { "Error" }
            p { "{props.message}" }
        }
    }
}
