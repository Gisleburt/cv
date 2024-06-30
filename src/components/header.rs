use dioxus::prelude::*;

#[component]
pub fn Header(name: String, title: String) -> Element {
    rsx!(
        header {
            h1 { {name} }
            h2 { {title} }
        }
    )
}
