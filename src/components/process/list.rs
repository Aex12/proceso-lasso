use dioxus::prelude::*;

use crate::process::Process;

#[component]
pub fn DxProcessLine (process: Process, selected_process: Signal<Option<Process>>) -> Element {
    let path = process.path.clone().map(|p| p.to_str().unwrap().to_owned()).unwrap_or(String::from(""));
    let is_selected = use_memo(move || selected_process().as_ref().map(|p| p.pid == process.pid).unwrap_or(false));
    let tr_class = use_memo(move || if is_selected() { "bg-gray-600 text-white cursor-pointer" } else { "cursor-pointer" });
    rsx!(
        tr {
            onclick: move |_| {
                println!("clicked");
                *selected_process.write() = if is_selected() { None } else { Some(process.clone()) };
            },
            class: tr_class,
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
    println!("rendered process list");

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
                        DxProcessLine { process: process, selected_process: selected_process }
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