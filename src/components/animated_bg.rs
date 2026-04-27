use dioxus::prelude::*;

#[component]
pub fn AnimatedBackground() -> Element {
    rsx! {
        div { class: "animated-bg",
            // Sub-Grid (Very faint)
            for i in 0..50 {
                div { 
                    class: "grid-line grid-line-v", 
                    style: "left: {i * 2}%; opacity: 0.1;" 
                }
            }
            // Vertical Grid Lines
            for i in 0..10 {
                div { 
                    class: "grid-line grid-line-v", 
                    style: "left: {i * 10}%" 
                }
            }
            // Horizontal Grid Lines
            for i in 0..10 {
                div { 
                    class: "grid-line grid-line-h", 
                    style: "top: {i * 10}%" 
                }
            }
            
            // Moving Pulse Lines
            div { class: "pulse-line", style: "left: 10%; animation-duration: 10s; animation-delay: 0s;" }
            div { class: "pulse-line", style: "left: 30%; animation-duration: 7s; animation-delay: 2s;" }
            div { class: "pulse-line", style: "left: 50%; animation-duration: 12s; animation-delay: 4s;" }
            div { class: "pulse-line", style: "left: 70%; animation-duration: 9s; animation-delay: 1s;" }
            div { class: "pulse-line", style: "left: 90%; animation-duration: 11s; animation-delay: 5s;" }
            
            div { class: "pulse-line pulse-line-h", style: "top: 20%; animation-duration: 15s; animation-delay: 0s;" }
            div { class: "pulse-line pulse-line-h", style: "top: 40%; animation-duration: 12s; animation-delay: 3s;" }
            div { class: "pulse-line pulse-line-h", style: "top: 60%; animation-duration: 18s; animation-delay: 6s;" }
            div { class: "pulse-line pulse-line-h", style: "top: 80%; animation-duration: 14s; animation-delay: 2s;" }
        }
    }
}
