use dioxus::prelude::*;
use std::rc::Rc;
use crate::domain::models::{NamingPattern, SplitConfig};

#[derive(Props, Clone, PartialEq)]
pub struct ConfigPanelProps {
    pub config: SplitConfig,
    pub total_pages: Option<usize>,
    pub on_config_changed: EventHandler<SplitConfig>,
}

#[component]
pub fn ConfigPanel(props: ConfigPanelProps) -> Element {
    let on_change = props.on_config_changed.clone();
    let total_pages = props.total_pages;
    let pages_per_file = props.config.pages_per_file;
    let output_dir = props.config.output_dir.clone();
    let naming = props.config.naming.clone();
    let overwrite = props.config.overwrite;
    let base_config = Rc::new(props.config.clone());

    rsx! {
        div { class: "config-panel",
            label { "Pages per file: {pages_per_file}" }

            match total_pages {
                Some(max) => rsx! {
                    input {
                        r#type: "range",
                        min: "1",
                        max: "{max}",
                        value: "{pages_per_file}",
                        oninput: {
                            let bc = base_config.clone();
                            move |evt: Event<FormData>| {
                                let ppf = evt.value().parse::<usize>().unwrap_or(2);
                                let new_config = SplitConfig {
                                    pages_per_file: ppf,
                                    ..(*bc).clone()
                                };
                                on_change.call(new_config);
                            }
                        }
                    }
                },
                None => rsx! {
                    input {
                        r#type: "number",
                        min: "1",
                        value: "{pages_per_file}",
                        oninput: {
                            let bc = base_config.clone();
                            move |evt: Event<FormData>| {
                                let ppf = evt.value().parse::<usize>().unwrap_or(2);
                                let new_config = SplitConfig {
                                    pages_per_file: ppf,
                                    ..(*bc).clone()
                                };
                                on_change.call(new_config);
                            }
                        }
                    }
                }
            }

            div { class: "config-row",
                label { "Output folder:" }
                span { "{output_dir.display()}" }
                button {
                    onclick: {
                        let bc = base_config.clone();
                        move |_| {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                let new_config = SplitConfig {
                                    output_dir: path,
                                    ..(*bc).clone()
                                };
                                on_change.call(new_config);
                            }
                        }
                    },
                    "Browse..."
                }
            }

            div { class: "config-row",
                label { "Naming:" }
                select {
                    onchange: {
                        let bc = base_config.clone();
                        move |evt: Event<FormData>| {
                            let naming = match evt.value().as_str() {
                                "range" => NamingPattern::WithPageRange,
                                "custom" => NamingPattern::Custom { prefix: "scan".into() },
                                _ => NamingPattern::Sequential,
                            };
                            let new_config = SplitConfig {
                                naming,
                                ..(*bc).clone()
                            };
                            on_change.call(new_config);
                        }
                    },
                    option { value: "sequential", selected: matches!(naming, NamingPattern::Sequential), "Sequential" }
                    option { value: "range", selected: matches!(naming, NamingPattern::WithPageRange), "With Page Range" }
                    option { value: "custom", selected: matches!(naming, NamingPattern::Custom { .. }), "Custom" }
                }
            }

            div { class: "config-row",
                label { "Overwrite:" }
                input {
                    r#type: "checkbox",
                    checked: "{overwrite}",
                    onchange: {
                        let bc = base_config.clone();
                        move |evt: Event<FormData>| {
                            let checked = evt.checked();
                            let new_config = SplitConfig {
                                overwrite: checked,
                                ..(*bc).clone()
                            };
                            on_change.call(new_config);
                        }
                    }
                }
            }
        }
    }
}
