use dioxus::prelude::*;

use crate::lasso::{Process, ProcessManager};
use crate::system::windows::WindowsProcessManager;

use super::components::{DxProcessList, Button, DxProcessOverview};

pub fn App() -> Element {
    let process_provider = WindowsProcessManager::new().unwrap();
    let process_list = process_provider.getProcessList().unwrap();
    let processes: Signal<Vec<Process>> = use_signal(|| process_list.processes().clone());
    let selected_process: Signal<Option<Process>> = use_signal(|| None);
    let process_is_selected: ReadOnlySignal<bool> = use_memo(move || selected_process.read().as_ref().is_some());

    rsx! {
        link { rel: "stylesheet", href: "public/tailwind.css" }
        div {
            class: "flex flex-col h-full w-full p-4",
            div {
                class: "max-w-full h-[70vh]",
                DxProcessList { processes, selected_process }
            }
            div {
                class: "flex flex-row space-between space-x-4 py-4",
                div {
                    Button { onclick: move |_| (), "Stop" }
                    Button { "Reload" }
                }
                if process_is_selected() {
                    DxProcessOverview { process: selected_process }
                }
            }
        }
    }
}

pub fn launch_app () {
    launch(App);
}