use dioxus::prelude::*;
use crate::domain::models::ChunkPlan;

#[derive(Props, Clone, PartialEq)]
pub struct ChunkPreviewProps {
    pub plan: ChunkPlan,
}

#[component]
pub fn ChunkPreview(props: ChunkPreviewProps) -> Element {
    rsx! {
        div { class: "chunk-preview",
            p { "Preview: {props.plan.total_files} files will be created" }
            div { class: "chunk-grid",
                for chunk in &props.plan.chunks {
                    div { class: "chunk-item",
                        "{chunk.start}–{chunk.end}"
                    }
                }
            }
        }
    }
}
