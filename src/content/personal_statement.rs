use crate::components::*;
use dioxus::prelude::*;
use indoc::indoc;

#[component]
pub fn PersonalStatement() -> Element {
    let statements = [
        rsx!(
            {"Throughout my five years in technical leadership I've always believed that"}
            em { " the real 10x engineers are those helping everyone around them achieve more " }
            {indoc! {"
                . This has been the north star of my leadership style; show my team what supporting
                each other looks like, give them the tools they need, and help them grow, and they
                will always deliver to their best.
            "}}
        ),
        rsx!({
            indoc! {"
                Hire me to build strong teams of people who deliver value through understanding
                the business requirements and the application of engineering excellence. I will
                achieve this through building confidence, trust and a culture of support and self
                development.
            "}
        }),
        rsx!({
            indoc! {"
                Self development is my goal too, I strive to keep learning how to be a better
                manager, particularly continuing to learn how to best align with other teams and
                work on larger projects.
            "}
        }),
    ];

    rsx!(
        Section { title: "Personal Statement",
            for statement in statements {
               p {{ statement }}
            }
        }
    )
}
