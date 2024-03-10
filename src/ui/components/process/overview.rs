use dioxus::prelude::*;

use crate::lasso::Process;

#[component]
pub fn DxProcessOverview (process: Process) -> Element {
    let path = process.path.clone().map(|p| p.to_str().unwrap().to_owned()).unwrap_or(String::from(""));

    rsx!(
        div {
            class: "border p-4 flex flex-col",
            p { "PID: {&process.pid}" }
            p { "Name: {&process.name}" }
            p { "Priority: {process.priority}" }
            p { "Path: {path}" }
        }
    )
}