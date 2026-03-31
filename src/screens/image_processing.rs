use dioxus::prelude::*;
use crate::application::app_state::AppState;

#[component]
pub fn ImageProcessingScreen(state: Signal<AppState>) -> Element {
    let current = state.read().clone();
    let images = current.image_config.as_ref().map(|c| c.images.len()).unwrap_or(0);

    rsx! {
        div { class: "splitting-screen",
            h2 { "Creating PDF..." }
            p { "Processing {images} images" }
            div { class: "progress-bar-container",
                div {
                    class: "progress-bar",
                    style: "width: 100%; animation: pulse 1.5s infinite"
                }
            }
            p { class: "current-file", "Please wait, this may take a moment..." }
        }
    }
}
