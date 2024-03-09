#![allow(non_snake_case)]
use dioxus::prelude::*;
use traits::ProcessManager;

pub mod traits;
pub mod structs;
pub mod providers;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let process_provider = providers::WindowsProcessManager::new()?;
    let process_list = process_provider.getProcessList()?;
    dbg!(process_list);
    // launch the dioxus app in a webview
    launch(App);
    Ok(())
}

pub fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}