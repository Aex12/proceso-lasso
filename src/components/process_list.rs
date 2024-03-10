use dioxus::prelude::*;

use crate::structs::Process;

#[component]
pub fn DxProcess (process: Process) -> Element {
    let path = process.path.map(|p| p.to_str().unwrap().to_owned()).unwrap_or(String::from(""));
    rsx!(
        tr {
            td { class: "text-center px-2 border", "{&process.pid}" }
            td { class: "border", "{&process.name}" }
            td { class: "px-2 border", "{process.priority}" }
            td { class: "px-2 border", "{path}" }
        }
    )
}

#[component]
pub fn DxProcessList (processes: Vec<Process>) -> Element {
    rsx!(
        div {
            class: "m-4 max-w-full max-h-full w-full h-full overflow-y-scroll overflow-x-auto",
            table {
                class: "table-fixed border",
                thead {
                    tr {
                        th { "PID" }
                        th { class: "text-left", "Name" }
                        th { class: "text-left", "Priority" }
                        th { class: "text-left", "Path" }
                    }
                }
                tbody {
                    for process in processes {
                        DxProcess { process: process }
                    }
                }
            }
        }
    )
}