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

const ROLES: &'static [Role<'static>] = &[
    Role {
        company: "Fio's Quest",
        link: None,
        title: "Founder",
        from_to: FromTo::const_new(Some("Jan '24"), None),
        highlights: highlight_list!(
            { "Taught" ["thousands of people"] "the basics of Rust" }
            { "Wrote an online book called" ["Idiomatic Rust in Simple Steps"] }
            // { "Developed a way to" ["teach the complexities of lifetimes"] "with kites" }
            { "Built and" ["monetised"] "greenfield apps and websites using Rust and Dioxus" }
            { "Created a company to" ["demystify the complexities" ] "of software engineering" }
            { "Produced" ["dozens of videos"] "to teach programming" }
        ),
    },
    Role {
        company: "Beamery",
        link: None,
        title: "Principal Engineer",
        from_to: FromTo::const_new(Some("Nov '22"), Some("Dec '23")),
        highlights: highlight_list!(
            { "Established a" ["working group of principal engineers"] "to manage alignment across multiple teams" }
            { ["Mentored and coached"] "engineers at all levels from Junior to Principal" }
            { "Authored a comprehensive review of" ["5 IDaaS providers"] }
            { "Architected a" ["passwordless auth system,"] "unlocking new revenue" }
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
                { "Managed the engineers for the internationalisation team" }
                { ["Influenced the wider business"] "to change practices to not require a specialised internationalisation team, pivoting my team to content" }
                { "Grew team" ["from 2 engineers to 8"] "and successfully advocated for and supported my staff through" ["6 promotions"] }
                { "Developed a new way to store and render content that was adopted by other teams, reducing the effort to produce a new page" ["from 10 days with 2 engineers, to half a day with no engineers"] }
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
            { "Formally coached a data scientist and informally coached several engineers" }
            { "Used responsive design to" ["mitigate a 12% loss on ad spend"] }
            { "Gave multiple " ["“lunch and learns”"] " (the most popular was on cryptography)" }
        ),
    },
    Role {
        company: "Apolitical Group Ltd",
        link: None,
        title: "Senior Software Engineer",
        from_to: FromTo::const_new(Some("Nov '17"), Some("Oct '19")),
        highlights: highlight_list!(
            { "Planned and developed the architecture that took a platform struggling to serve 1000 users to" ["38,000 when I left"] "and" ["over 250,000"] "today" }
            { "Built a Rust service that was" ["4x faster and 5x more memory efficient"] "than an equivalent Node service" }
            { "Coached two junior engineers, including" ["teaching them Rust"] }
            { "Designed, documented and deployed the CI/CD processes" }
            { "Developed a data backup system that not only kept data safe but could be " ["used to test new code"] " before deploying to production" }
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
                    class: "title",
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
            class: "title",
            em { "More on request" }
        }
    )
}
