use crate::components::*;
use dioxus::prelude::*;
use indoc::indoc;

#[component]
pub fn PersonalStatement() -> Element {
    let statements = [
        rsx!(
            p {
                {indoc!{"
                    Early in my career, a colleague was at risk of redundancy, a competent engineer
                    from a different team. At the same time, our team was hiring.
                "}}
            }
        ),
        rsx!(
            p {
                {indoc!{"
                    I successfully petitioned to train the other engineer and was given eight weeks
                    to show them the ropes, after which they joined my team, being immediately
                    productive, saving the business money in redundancy and hiring.
                "}}
            }
        ),
        rsx!(
            p {
                {indoc!{"
                    That engineers' success is their own, but it taught me to understand a business
                    holistically. It stoked my passion to help those around me grow, I believe
                "}},
                em { "the real 10x engineers are those helping everyone around them achieve more." }
            }
        ),
    ];

    rsx!(
        Section { title: "Personal Statement",
            for statement in statements {
                { statement }
            }
        }
    )
}
