use crate::components::*;
use dioxus::prelude::*;

#[derive(Copy, Clone)]
enum LinkType {
    Web,
    Tel,
    Mail,
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

#[component]
pub fn ContactDetails() -> Element {
    let contact_details = [
        ContactDetail::new(LinkType::Tel, "Mobile", "+44 7838 200176"),
        ContactDetail::new(LinkType::Web, "Web", "danielmason.com"),
        ContactDetail::new(LinkType::Mail, "Email", "daniel@danielmason.com"),
        ContactDetail::new(LinkType::Web, "GitHub", "github.com/gisleburt"),
        ContactDetail::new(
            LinkType::Web,
            "Linked In",
            "linkedin.com/in/danieljamesmason",
        ),
    ];

    let all_details = contact_details.iter().map(|contact_detail| {
        let link_prefix = match contact_detail.link_type {
            LinkType::Web => "https://",
            LinkType::Tel => "tel:",
            LinkType::Mail => "mailto:",
        };
        rsx!(
            dt { "{contact_detail.label}:" }
            dd {
                a { href: "{link_prefix}{contact_detail.link}", {contact_detail.link} }
            }
        )
    });

    rsx!(
        Section { title: "Contact Details",
            dl { {all_details} }
        }
    )
}
