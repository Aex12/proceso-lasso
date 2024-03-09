#![allow(non_snake_case)]
use dioxus::prelude::*;

pub mod traits;
pub mod structs;
pub mod providers;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
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