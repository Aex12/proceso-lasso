/* Prevents additional console window on Windows in release */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
use dioxus::prelude::*;
use config::{ConfigManager, YamlConfigManager};
use process::{ProcessManager, WindowsProcessManager};

pub mod components;
pub mod config;
pub mod process;
pub mod lasso;

use components::{DxProcessList, Button};

fn main() -> () {
    let mut config_manager = YamlConfigManager::new("config.yaml");
    let config = config_manager.get().unwrap();
    // put after get to ensure the file is created if it doesn't exist
    config_manager.put(config.clone()).unwrap();

    let process_manager = WindowsProcessManager::new().unwrap();
    let process_list = process_manager.getProcessList().unwrap();
    process_list.processes().iter().for_each(|process| {
        let preset_name: &str = config.find_rule(process).map(|rule| &rule.preset)
            .unwrap_or_else(|| &config.default_preset);
        let preset = config.get_preset(preset_name).unwrap();
        if let Some(affinity) = &preset.affinity {
            match process_manager.setProcessAffinity(process.pid, affinity) {
                Ok(_) => println!("{}: {} {}", process.name, preset_name, affinity),
                Err(e) => println!("Error setting affinity for process {}: {}", process.name, e),
            }
        }
    });

    // launch the dioxus app in a webview
    launch(App);
}

pub fn App() -> Element {
    let process_provider = WindowsProcessManager::new().unwrap();
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