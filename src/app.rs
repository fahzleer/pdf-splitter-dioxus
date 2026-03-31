use dioxus::prelude::*;
use crate::application::app_state::AppState;
use crate::components::error_display::ErrorDisplay;
use crate::screens::done::DoneScreen;
use crate::screens::file_loaded::FileLoadedScreen;
use crate::screens::image_processing::ImageProcessingScreen;
use crate::screens::image_setup::ImageSetupScreen;
use crate::screens::splitting::SplittingScreen;
use crate::screens::welcome::WelcomeScreen;

#[component]
pub fn App() -> Element {
    let state = use_signal(AppState::new);

    rsx! {
        div { class: "app",
            match state.read().screen.clone() {
                crate::domain::models::AppScreen::Welcome => rsx! {
                    WelcomeScreen { state }
                },
                crate::domain::models::AppScreen::FileLoaded => rsx! {
                    FileLoadedScreen { state }
                },
                crate::domain::models::AppScreen::Splitting => rsx! {
                    SplittingScreen { state }
                },
                crate::domain::models::AppScreen::Done => rsx! {
                    DoneScreen { state }
                },
                crate::domain::models::AppScreen::Error { msg } => rsx! {
                    ErrorDisplay { message: msg }
                },
                crate::domain::models::AppScreen::ImageSetup => rsx! {
                    ImageSetupScreen { state }
                },
                crate::domain::models::AppScreen::ImageProcessing => rsx! {
                    ImageProcessingScreen { state }
                },
            }
        }
    }
}
