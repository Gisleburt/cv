use crate::components::*;
use dioxus::prelude::*;

#[derive(Copy, Clone)]
struct Qualification<'a> {
    what: &'a str,
    from: &'a str,
}

impl<'a> Qualification<'a> {
    fn new(what: &'a str, from: &'a str) -> Self {
        Qualification { what, from }
    }
}

#[component]
pub fn Education() -> Element {
    let qualifications = [
        Qualification::new(
            "MSc Computer Games Technology",
            "Abertay University, Dundee",
        ),
        Qualification::new("BSc Cybernetics and Virtual Worlds", "Bradford University"),
    ];

    let all_qualifications = qualifications
        .iter()
        .map(|qualification| rsx!(
            dt { {qualification.what} }
            dd { {qualification.from} }
        ));

    rsx!(
        Section { title: "Education",
            dl { {all_qualifications} }
        }
    )
}
