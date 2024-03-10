/* Prevents additional console window on Windows in release */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
use dioxus::prelude::*;
use traits::{ConfigManager, ProcessManager};

pub mod traits;
pub mod structs;
pub mod providers;
pub mod components;

use components::{DxProcessList, Button};

fn main() -> () {
    let mut config_manager = providers::YamlConfigManager::new("config.yaml");
    let config = config_manager.get().unwrap();
    // put after get to ensure the file is created if it doesn't exist
    config_manager.put(config.clone()).unwrap();
    dbg!(config);
    // launch the dioxus app in a webview
    launch(App);
}

pub fn App() -> Element {
    let process_provider = providers::WindowsProcessManager::new().unwrap();
    let process_list = process_provider.getProcessList().unwrap();

    rsx! {
        link { rel: "stylesheet", href: "public/tailwind.css" }
        div {
            class: "w-[97vw] h-[70vh]",
            DxProcessList { processes: process_list.processes().clone() }
        }
        div {
            class: "flex flex-row space-x-4",
            Button { onclick: move |_| (), "Stop" }
            Button { "Reload" }
        }
    }
}