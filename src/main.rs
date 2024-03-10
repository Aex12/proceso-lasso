/* Prevents additional console window on Windows in release */
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
pub mod components;
pub mod process;
pub mod lasso;

use dioxus::prelude::*;

use lasso::{ConfigManager, YamlConfigManager};
use process::{ProcessManager, WindowsProcessManager};
use components::{DxProcessList, Button};

use crate::{components::DxProcessOverview, process::Process};

fn main() -> () {
    let mut config_manager = YamlConfigManager::new("config.yaml");
    let config = config_manager.get().unwrap();
    // put after get to ensure the file is created if it doesn't exist
    config_manager.put(config.clone()).unwrap();

    let process_manager = WindowsProcessManager::new().unwrap();
    config.apply(&process_manager).unwrap();

    // dispose config_manager, config, and process_manager
    drop(config_manager);
    drop(config);
    drop(process_manager);

    launch(App);
}

pub fn App() -> Element {
    let process_provider = WindowsProcessManager::new().unwrap();
    let process_list = process_provider.getProcessList().unwrap();
    let processes: Signal<Vec<Process>> = use_signal(|| process_list.processes().clone());
    let selected_process: Signal<Option<Process>> = use_signal(|| None);

    rsx! {
        link { rel: "stylesheet", href: "public/tailwind.css" }
        div {
            class: "flex flex-col h-full w-full p-4",
            div {
                class: "max-w-full h-[70vh]",
                DxProcessList { processes, selected_process }
            }
            div {
                class: "flex flex-row space-x-4 py-4",
                Button { onclick: move |_| (), "Stop" }
                Button { "Reload" }
                if selected_process.read().as_ref().is_some() {
                    DxProcessOverview { process: selected_process.read().clone().unwrap() }
                }
            }
        }
    }
}