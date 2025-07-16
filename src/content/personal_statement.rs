use crate::components::*;
use dioxus::prelude::*;

macro_rules! statement {
    ( $( { $($tokens:tt)+ } )+ ) => {
        rsx!(
            $(
                p { $({ statement_helper!($tokens) })+ }
            )+
        )
    };
}

macro_rules! statement_helper {
    ($normal:literal) => {
        $normal
    };
    ([$emphasis:literal]) => {
        rsx!(em { $emphasis })
    };
}

#[component]
pub fn PersonalStatement() -> Element {
    let statements = statement!(
        {
            "Throughout my many years in technical leadership, I've always believed that"
            [" the real 10x engineers are those helping everyone around them achieve more"]
            ". This has been the north star of my leadership style; show my team what  supporting "
            "each other looks like, give them the tools they need, and help them grow, and they "
            "will always deliver to their best ability."
        }

        {
            "Hire me to build strong teams of people who deliver value through understanding the "
            "business requirements and the application of engineering excellence. I will achieve "
            "this through building confidence, trust and a culture of support and self-development."
        }

        {
             "Despite my success, self-development remains my goal too. I strive to keep learning "
             "how to be a better manager, particularly continuing to learn how to best align with "
             "other teams and work on larger projects."
        }
    );

    rsx!(
        Section { title: "Personal Statement",
            {statements}
        }
    )
}
