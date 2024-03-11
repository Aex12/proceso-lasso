use dioxus::prelude::*;

use crate::{lasso::{Preset, Process}, ui::components::Button};

pub fn get_process_name (process: Option<&Process>) -> String {
    process.map(|p| p.name.clone()).unwrap_or(String::from(""))
}
pub fn get_process_path (process: Option<&Process>) -> String {
    process.map(
        |p| p.path
            .as_ref()
            .map(|p| p.to_str().unwrap().to_owned())
            .unwrap_or(String::from(""))
    ).unwrap_or(String::from(""))
}

#[component]
pub fn DxProcessOverview (process: Signal<Option<Process>>) -> Element {
    let mut name_rule = use_signal(move || get_process_name(process.read().as_ref()));
    let mut path_rule = use_signal(move || get_process_path(process.read().as_ref()));

    use_effect(move || {
        if let Some(process) = process.read().as_ref() {
            *name_rule.write() = get_process_name(Some(&process));
            *path_rule.write() = get_process_path(Some(&process));
        }
    });
    let mut preset: Signal<Option<Preset>> = use_signal(|| None);

    rsx!(
        div {
            class: "p-2 flex flex-col space-x-4",
            div {
                class: "flex flex-row",
                input {
                    class: "border border-gray-400 p-2",
                    value: name_rule,
                    oninput: move |e| { *name_rule.write() = e.value(); }
                }
                Button {
                    onclick: move |_| {
                        println!("adding name rule {}", name_rule.read());
                    },
                    "Add name rule"
                }
            }
            div {
                class: "flex flex-row",
                input {
                    class: "border border-gray-400 p-2",
                    value: path_rule,
                    oninput: move |e| { *path_rule.write() = e.value(); }
                }
                Button {
                    onclick: move |_| {
                        println!("adding path rule {}", path_rule.read());
                    },
                    "Add path rule"
                }
            }
        }
    )
}