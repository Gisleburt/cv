use dioxus::prelude::*;

#[component]
pub fn Section(title: String, children: Element) -> Element {
    rsx!(
        section {
            h2 { {title} }
            {children}
        }
    )
}
