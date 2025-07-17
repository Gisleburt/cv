use crate::components::*;
use dioxus::prelude::*;
use crate::transforms::ToElement;

#[derive(Copy, Clone)]
enum LinkType {
    Web,
    Tel,
    Mail,
}

impl LinkType {
    pub fn prefix(&self) -> &'static str {
        match self {
            LinkType::Web => "https://",
            LinkType::Tel => "tel:",
            LinkType::Mail => "mailto:",
        }
    }
}

struct ContactDetail<'a> {
    link_type: LinkType,
    label: &'a str,
    link: &'a str,
}

impl<'a> ContactDetail<'a> {
    fn new(link_type: LinkType, label: &'a str, link: &'a str) -> Self {
        ContactDetail {
            link_type,
            label,
            link,
        }
    }
}

impl<'a> ToElement for ContactDetail<'a> {
    fn to_element(&self) -> Element {
        let link_prefix = self.link_type.prefix();
        
        rsx!(
            dt { "{self.label}:" }
            dd {
                a { href: "{link_prefix}{self.link}", {self.link} }
            }
        )
    }
}

#[component]
pub fn ContactDetails() -> Element {
    let contact_details = [
        ContactDetail::new(LinkType::Tel, "Mobile", "+44 7838 200176"),
        ContactDetail::new(LinkType::Web, "Web", "danielmason.com"),
        ContactDetail::new(LinkType::Mail, "Email", "daniel@danielmason.com"),
        ContactDetail::new(LinkType::Web, "GitHub", "github.com/gisleburt"),
        ContactDetail::new(
            LinkType::Web,
            "LinkedIn",
            "linkedin.com/in/danieljamesmason",
        ),
    ];

    let contact_details_elements = contact_details.iter().map(ToElement::to_element);

    rsx!(
        Section { title: "Contact Details",
            dl { {contact_details_elements} }
        }
    )
}
