use crate::components::*;
use crate::transforms::*;
use dioxus::prelude::*;

#[derive(Copy, Clone)]
struct FromTo<'a> {
    from: Option<&'a str>,
    to: Option<&'a str>,
}

impl<'a> FromTo<'a> {
    pub const fn const_new(
        from: Option<&'static str>,
        to: Option<&'static str>,
    ) -> FromTo<'static> {
        FromTo { from, to }
    }
}

impl<'a> ToElement for FromTo<'a> {
    fn to_element(&self) -> Element {
        if self.from.is_none() && self.to.is_none() {
            return rsx!();
        }
        let divide = self.from.and(self.to).map(|_| " - ");
        rsx!(
            span {
                {self.from},
                {divide},
                {self.to}
            }
        )
    }
}

#[derive(Copy, Clone)]
struct RoleLink<'a> {
    title: &'a str,
    href: &'a str,
}

impl<'a> ToElement for RoleLink<'a> {
    fn to_element(&self) -> Element {
        rsx!(
            a { href: self.href, {self.title} }
        )
    }
}

enum Highlight<'a> {
    Normal(&'a str),
    Emphasis(&'a str),
}

impl<'a> ToElement for Highlight<'a> {
    fn to_element(&self) -> Element {
        match self {
            Highlight::Normal(text) => rsx!(" {text} "),
            Highlight::Emphasis(text) => rsx!(
                em { {text} }
            ),
        }
    }
}

macro_rules! highlight_list {
    ( $( { $($tokens:tt)+ } )+ ) => {
        &[
            $(
                &[
                    $( highlight_list_item_part!($tokens), )+
                ],
            )+
        ]
    };
}

macro_rules! highlight_list_item_part {
    ($normal:literal) => {
        &Highlight::Normal($normal)
    };
    ([$emphasis:literal]) => {
        &Highlight::Emphasis($emphasis)
    };
}

struct Role<'a> {
    company: &'a str,
    link: Option<RoleLink<'a>>,
    title: &'a str,
    from_to: FromTo<'a>,
    highlights: &'a [&'a [&'a Highlight<'a>]],
}

// ToDo: Investigate if I can replace the big highlights lists with a const nom parser
const ROLES: &'static [Role<'static>] = &[
    Role {
        company: "Fio's Quest",
        link: None,
        title: "Founder",
        from_to: FromTo::const_new(Some("Jan '24"), None),
        highlights: highlight_list!(
            { "Created a company to" ["demystify Rust"] "and other complex software engineering challenges" }
            { "Wrote an online book called" ["Idiomatic Rust in Simple Steps"] }
            { "Taught" ["thousands of people"] "the basics of Rust" }
            { "Built and" ["monetised"] "multiple greenfield apps and websites using Rust and Dioxus" }
        ),
    },
    Role {
        company: "Beamery",
        link: None,
        title: "Principal Engineer",
        from_to: FromTo::const_new(Some("Nov '22"), Some("Dec '23")),
        highlights: highlight_list!(
            { "Mentored and coached engineers at all levels" }
            { "Established a" ["working group of principal engineers"] "to manage alignment across multiple teams" }
            { "Authored a comprehensive review of" ["5 IDaaS providers"] "before architecting a passwordless auth system" }
            { ["Led a team of engineers"] "to build the candidate auth system" }
            { "Joined" ["leadership teams"] " for the Frontend and Backend Tribes" }
            { "Presented" ["3 “lunch and learns”"] "(all of which are on danielmason.com)" }
            { "Helped team go from" ["50% test coverage to 80%"] "reducing out of hours outages to zero and improving morale" }
        ),
    },
    Role {
        company: "Peloton",
        link: None,
        title: "Engineering Manager",
        from_to: FromTo::const_new(Some("Sept '20"), Some("Nov '22")),
        highlights: highlight_list!(
                { "Led the internationalisation team, then " ["influenced the wider business"] "to change practices to not require a specialised team, pivoting my team to content" }
                { "Grew team" ["from 2 engineers to 8"] "and successfully processed" ["6 promotions"] }
                { "Developed a new way to store and render content that was adopted by other teams, reducing the effort to produce a new page" ["from 10 days with 2 engineers, to 0.5 days with 0 engineers"] }
                { "Optimised project management to  improve reliability of estimations," ["reduced whole team meetings by 75%"] "and increased team happiness" }
                { "Stayed on top of reports’ personal goals to ensure they achieved them and helped them progress their careers" }
                { "Cultivated" ["psychological safety"] "and prioritised team member growth" }
                { "Successfully coached the manager who succeeded me" }
        ),
    },
    Role {
        company: "Triptease Ltd",
        link: None,
        title: "Senior Software Engineer",
        from_to: FromTo::const_new(Some("Oct '19"), Some("Sept '20")),
        highlights: highlight_list!(
            { "Immediately" ["resolved a bug"] "that caused one of their systems to be" ["unavailable for 45mins every day"] }
            { "Formally career coached a data scientist and informally coached several engineers" }
            { "Used responsive design to" ["mitigate losses on ad spend of ~12%"] }
            { "Gave multiple “lunch and learns” (the most popular of which covered the impact of thirsty Victorians on modern cryptography)" }
        ),
    },
    Role {
        company: "Apolitical Group Ltd",
        link: None,
        title: "Senior Software Engineer",
        from_to: FromTo::const_new(Some("Nov '17"), Some("Oct '19")),
        highlights: highlight_list!(
            { "Planned and developed the architecture that took a platform struggling service to" ["over 38x the number of users"] "it had previously supported" }
            { "Built a Rust service that was" ["4x faster and 5x more memory efficient"] "than an equivalent Node service" }
            { "Coached two junior engineers, including" ["teaching them Rust"] }
            { "Designed, documented and deployed the Continuous Deployment processes" }
        ),
    },
    Role {
        company: "MOO Print Ltd",
        link: None,
        title: "Software Engineer",
        from_to: FromTo::const_new(Some("May '16"), Some("Nov '17")),
        highlights: highlight_list!(
            { "Successfully advocated for and coached a junior engineer at risk of redundancy so that they could join our team" }
            { "Evangelised Continuous Deployment, participated in the CD working group and" ["influenced business wide decisions"] }
            {"Took a greenfield project to production"}
        ),
    },
];

#[component]
pub fn Experience() -> Element {
    let roles = ROLES.iter().map(|role| {
        rsx!(
            div {
                h3 {
                    em { {role.company} }
                    " - "
                    {role.title},
                    {role.link.map(|link| link.to_element())},
                    {role.from_to.to_element()}
                }
                ul {
                    for highlight in role.highlights {
                        li {
                            for part in highlight {
                                {part.to_element()}
                            }
                        }
                    }
                }
            }
        )
    });
    rsx!(
        Section { title: "Experience" }
        {roles},
        h3 {
            em { "More on request" }
        }
    )
}
