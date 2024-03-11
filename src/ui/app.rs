use dioxus::prelude::*;

use crate::lasso::Process;
use crate::system::{create_config_store, create_process_manager};

use super::components::{DxProcessList, Button, DxProcessOverview};

pub fn App() -> Element {
    let process_manager = use_memo(|| create_process_manager());
    let config_store = use_memo(|| create_config_store("yaml", "config.yaml"));

    let process_list = use_signal(|| process_manager.read().get_process_list().unwrap());
    let config = use_signal(|| config_store.read().get().unwrap());

    use_effect(move || {
        let process_manager = process_manager.read();
        let config = config.read();
        process_manager.apply(&config).unwrap();
    });

    let selected_process: Signal<Option<Process>> = use_signal(|| None);

    rsx! {
        link { rel: "stylesheet", href: "public/tailwind.css" }
        div {
            class: "flex flex-col h-full w-full p-4",
            div {
                class: "max-w-full h-[70vh]",
                DxProcessList { process_list, selected_process }
            }
            div {
                class: "flex flex-row space-between space-x-4 py-4",
                div {
                    Button { onclick: move |_| (), "Stop" }
                    Button { "Reload" }
                }
                if selected_process.read().as_ref().is_some() {
                    DxProcessOverview { process: selected_process }
                }
            }
        }
    }
}

pub fn launch_app () {
    launch(App);
}