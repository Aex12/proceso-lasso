use dioxus::prelude::*;

use crate::lasso::Process;

#[component]
pub fn DxProcessLine (process: Process, selected: bool, on_click: EventHandler<()>) -> Element {
    let path = process.path.clone().map(|p| p.to_str().unwrap().to_owned()).unwrap_or(String::from(""));
    rsx!(
        tr {
            onclick: move |_| on_click.call(()),
            class: if selected { "bg-gray-600 text-white cursor-pointer" } else { "cursor-pointer" },
            td { class: "text-center px-2 border", "{&process.pid}" }
            td { class: "border", "{&process.name}" }
            td { class: "px-2 border", "{process.priority}" }
            td { class: "px-2 border", "{path}" }
        }
    )
}

#[component]
pub fn DxProcessList (processes: Signal<Vec<Process>>, selected_process: Signal<Option<Process>>) -> Element {
    let mut search_term = use_signal(|| String::from(""));
    let filtered_processes: ReadOnlySignal<Vec<Process>> = use_memo(move || {
        match search_term.read().as_str() {
            "" => processes.read().clone(),
            term @ _ => processes.read().iter().filter(|p| p.name.contains(term)).cloned().collect(),
        }
    });

    rsx!(div {
        class: "max-w-full max-h-full w-full h-full flex flex-col",
        div {
            class: "max-w-full max-h-full flex-grow overflow-y-scroll overflow-x-auto",
            table {
                class: "table-fixed border",
                thead {
                    tr {
                        th { "PID" }
                        th { class: "text-left", "Name" }
                        th { class: "text-left", "Preset" }
                        th { class: "text-left", "Path" }
                    }
                }
                tbody {
                    for process in filtered_processes.read().clone() {
                        DxProcessLine {
                            selected: selected_process.read().as_ref().map(|p| p.pid).unwrap_or(-1) == process.pid,
                            process: process.clone(),
                            on_click: move |_| {
                                let selected = selected_process.read().as_ref().map(|p| p.pid).unwrap_or(-1) == process.pid;
                                *selected_process.write() = if selected { None } else { Some(process.clone()) };
                            }
                        }
                    }
                }
            }
        }
        input {
            class: "w-full p-2 border",
            oninput: move |e| {
                *search_term.write() = e.value();
            },
            placeholder: "Search"
        }
    })
}