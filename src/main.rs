/* Prevents additional console window on Windows in release */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
use dioxus::prelude::*;
use traits::ProcessManager;

pub mod traits;
pub mod structs;
pub mod providers;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() -> () {
    let process_provider = providers::WindowsProcessManager::new().unwrap();
    let process_list = process_provider.getProcessList().unwrap();
    dbg!(process_list);
    // launch the dioxus app in a webview
    launch(App);
}

pub fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}