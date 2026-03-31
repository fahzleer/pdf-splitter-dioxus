mod app;
mod application;
mod components;
mod domain;
mod infrastructure;
mod screens;

fn main() {
    dioxus::launch(app::App);
}
