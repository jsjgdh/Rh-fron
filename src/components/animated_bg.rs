use dioxus::prelude::*;

#[component]
pub fn AnimatedBackground() -> Element {
    rsx! {
        div { class: "animated-bg" }
    }
}
