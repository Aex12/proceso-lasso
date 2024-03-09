/* Prevents additional console window on Windows in release */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
use dioxus::prelude::*;
use traits::ProcessManager;

pub mod traits;
pub mod structs;
pub mod providers;
pub mod components;

use components::{DxProcessList, Button};

fn main() -> () {
    // launch the dioxus app in a webview
    launch(App);
}

pub fn App() -> Element {
    let mut count = use_signal(|| 0);
    let process_provider = providers::WindowsProcessManager::new().unwrap();
    let process_list = process_provider.getProcessList().unwrap();

    rsx! {
        link { rel: "stylesheet", href: "public/tailwind.css" }
        h1 { "High-Five counter: {count}" }
        Button { onclick: move |_| count += 1, "Up high!" }
        Button { onclick: move |_| count -= 1, "Down low!" }
        DxProcessList { processes: process_list.processes().clone() }
    }
}