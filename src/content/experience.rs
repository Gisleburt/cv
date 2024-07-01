use crate::components::*;
use dioxus::prelude::*;

trait ToElement {
    fn to_element(&self) -> Element;
}

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

    pub const fn const_default() -> FromTo<'static> {
        FromTo {
            from: None,
            to: None,
        }
    }
}

impl<'a> ToElement for FromTo<'a> {
    fn to_element(&self) -> Element {
        if self.from.is_none() && self.to.is_none() {
            return rsx!(  );
        }
        let divide = self.from.and(self.to).map(|_| " - ");
        rsx!(
            br {}
            {self.from},
            {divide},
            {self.to}
        )
    }
}

#[derive(Copy, Clone)]
struct RoleLink<'a> {
    title: &'a str,
    href: &'a str,
}

impl<'a> RoleLink<'a> {
    pub fn render(&self) -> Element {
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
            Highlight::Normal(text) => rsx!( " {text} " ),
            Highlight::Emphasis(text) => rsx!(
                em { {text} }
            ),
        }
    }
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
        link: Some(RoleLink{ title: "fios-quest.com", href: "https://fios-quest.com"}),
        title: "Founder",
        from_to: FromTo::const_default(),
        highlights: &[
            &[
                &Highlight::Normal("Grew a YouTube channel to"),
                &Highlight::Emphasis("over 500 subscribers in just 8 videos"),
            ],
            &[
                &Highlight::Normal("Generated"),
                &Highlight::Emphasis("12 chapters of innovative learning materials"),
                &Highlight::Normal("to teach the Rust programming language in simple steps"),
            ],
            &[&Highlight::Normal("Incorporated a company to manage revenue")],
            &[
                &Highlight::Normal("Built the website using Dioxus, a"),
                &Highlight::Emphasis("state of the art frontend framework"),
                &Highlight::Normal("of which Daniel is an"),
                &Highlight::Emphasis("early adopter"),
            ],
            &[&Highlight::Normal("Self taught videography and editing using Davinci Resolve")],
        ],
    },
    Role {
        company: "Beamery",
        link: None,
        title: "Principal Engineer",
        from_to: FromTo::const_new(Some("Nov '22"), Some("Dec '23")),
        highlights: &[
            &[&Highlight::Normal("Mentored and Coached engineers at all levels")],
            &[
                &Highlight::Normal("Established a"),
                &Highlight::Emphasis("working group of 5 principal engineers"),
                &Highlight::Normal("to manage alignment across multiple teams"),
            ],
            &[
                &Highlight::Normal("Authored a comprehensive review of"),
                &Highlight::Emphasis("5 IDaaS providers"),
                &Highlight::Normal("before architecting a passwordless auth system"),
            ],
            &[
                &Highlight::Emphasis("Lead a team of 4 engineers"),
                &Highlight::Normal("to build the candidate auth system"),
            ],
            &[&Highlight::Normal("Joined the leadership teams for Beamery’s Frontend and Backend Tribes")],
            &[
                &Highlight::Normal("Presented"),
                &Highlight::Emphasis("3 “lunch and learns”"),
                &Highlight::Normal("on various topics (all of which are on danielmason.com)"),
            ],
            &[
                &Highlight::Normal("Helped team go from"),
                &Highlight::Emphasis("50% test coverage to 80%"),
                &Highlight::Normal("reducing out of hours outages to zero and improving morale"),
            ],
        ],
    },
    Role {
        company: "Peloton",
        link: None,
        title: "Engineering Lead / Engineering Manager",
        from_to: FromTo::const_new(Some("Sept '20"), Some("Nov '22")),
        highlights: &[
            &[
                &Highlight::Normal("Led the internationalisation team, then "),
                &Highlight::Normal("influenced the wider business"),
                &Highlight::Normal("to change practices to not require a specialised team, pivoting my team to content"),
            ],
            &[
                &Highlight::Normal("Grew the team"),
                &Highlight::Emphasis("from 2 engineers to 8 engineers"),
            ],
            &[
                &Highlight::Normal("Developed a new way to store and render content that was adopted by other teams, reducing the effort to produce a new page"),
                &Highlight::Emphasis("from 10 days with 2 engineers, to 0.5 days with 0 engineers"),
            ],
            &[
                &Highlight::Normal("Optimised project management to  improve reliability of estimations,"),
                &Highlight::Emphasis("reduced whole team meetings by 75%"),
                &Highlight::Normal("and increased team happiness"),
            ],
            &[&Highlight::Normal("Stayed on top of reports’ personal goals to ensure they achieved them and helped them progress their careers")],
            &[
                &Highlight::Normal("Cultivated an environment of"),
                &Highlight::Emphasis("psychological safety"),
                &Highlight::Normal("and prioritised team growth"),
            ],
            &[&Highlight::Normal("Successfully coached the manager who replaced me")],
        ],
    },
    Role {
        company: "Triptease Ltd",
        link: None,
        title: "Senior Software Engineer",
        from_to: FromTo::const_new(Some("Oct '19"), Some("Sept '20")),
        highlights: &[
            &[
                &Highlight::Emphasis("Immediately"),
                &Highlight::Normal("resolved a bug"),
                &Highlight::Emphasis("that caused one of their systems to be"),
                &Highlight::Normal("unavailable for 45mins every day"),
            ],
            &[&Highlight::Normal("Formally career coached a data scientist and informally coached several engineers")],
            &[&Highlight::Normal("Gave multiple “lunch and learns” (the most popular of which covered the impact of thirsty Victorians on modern cryptography)")],
            &[
                &Highlight::Normal("Created a novel resolution for an advertising bug"),
                &Highlight::Emphasis("mitigating losses on ad spend of ~12%"),
            ],
        ],
    },
    Role {
        company: "Apolitical Group Ltd",
        link: None,
        title: "Senior Software Engineer",
        from_to: FromTo::const_new(Some("Nov '17"), Some("Oct '19")),
        highlights: &[
            &[
                &Highlight::Normal("Planned and developed the platform architecture that took a service struggling service to"),
                &Highlight::Emphasis("over 38x the number of users"),
                &Highlight::Normal("it had previously supported"),
            ],
            &[
                &Highlight::Normal("Built a Rust service that was"),
                &Highlight::Emphasis("4x faster and 5x more memory efficient"),
                &Highlight::Normal("than an equivalent Node service"),
            ],
            &[
                &Highlight::Normal("Coached two junior engineers, including"),
                &Highlight::Emphasis("teaching them Rust"),
            ],
            &[&Highlight::Normal("Designed, documented and deployed the Continuous Deployment processes")],
        ],
    },
    Role {
        company: "MOO Print Ltd",
        link: None,
        title: "Software Engineer",
        from_to: FromTo::const_new(Some("May '16"), Some("Nov '17")),
        highlights: &[
            &[&Highlight::Normal("Successfully advocated for and coached a junior engineer at risk of redundancy so that they could join our team")],
            &[&Highlight::Normal("Took a greenfield project to production")],
            &[
                &Highlight::Normal("Evangelized Continuous Deployment, participated in the CD working group and"),
                &Highlight::Emphasis("influenced business wide decisions"),
            ],
        ],
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
                    {role.link.map(|link| link.render())},
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
        {roles}
    )
}
