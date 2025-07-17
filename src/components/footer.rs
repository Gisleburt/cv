use dioxus::prelude::*;
use crate::content::ContactDetails;

#[component]
pub fn Footer(children: Element) -> Element {
    rsx!(
        footer {
            {children}
        }
    )
}
