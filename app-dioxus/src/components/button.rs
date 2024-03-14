use dioxus::prelude::*;

#[component]
pub fn Button(onclick: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
    rsx!(
        button {
            class: "px-4 py-2 border bg-gray-200 border-gray-400 rounded-lg",
            onclick: move |evt| {
                if let Some(onclick) = &onclick {
                    onclick.call(evt);
                }
            },
            {children}
        }
    )
}