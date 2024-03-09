use dioxus::prelude::*;

#[component]
pub fn Button(onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx!(
        button {
            class: "px-4 py-2 border",
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    )
}