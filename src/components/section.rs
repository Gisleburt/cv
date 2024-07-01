use crate::transforms::create_safe_name;
use dioxus::prelude::*;

#[component]
pub fn Section(title: String, children: Element) -> Element {
    let id = create_safe_name(&title);
    rsx!(
        section { id,
            h2 { {title} }
            {children}
        }
    )
}
