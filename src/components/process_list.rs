use dioxus::prelude::*;

use crate::structs::Process;

#[component]
pub fn DxProcess (process: Process) -> Element {
    rsx!(
        div {
            class: "flex flex-row border p-2 w-full h-fit space-between space-x-2",
            p { "[{&process.pid}]" }
            h1 { "{&process.name}" }
            p { "Priority: {process.priority}" }
            p { "Path: {&process.path.as_ref().unwrap_or(&String::from(\"\"))}" }
        }
    )
}

#[component]
pub fn DxProcessList (processes: Vec<Process>) -> Element {
    rsx!(
        div {
            h1 { class: "text-red-500", "Process List" }
            div {
                class: "flex flex-col border m-4 w-[95vw] h-[60vh] overflow-y-scroll overflow-x-auto",
                for process in processes {
                    DxProcess { process: process }
                }
            }
        }
    )
}