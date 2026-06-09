use dioxus::prelude::*;

pub fn rem(value: f32) -> String {
    format!("{}rem", value)
}

#[component]
pub fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}