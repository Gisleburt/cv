use crate::components::*;
use dioxus::prelude::*;

#[derive(Copy, Clone)]
struct RoleLink<'a> {
    title: &'a str,
    href: &'a str,
}

impl<'a> RoleLink<'a> {
    pub fn render(&self) -> Element {
        rsx!(a { href: self.href, {self.title}})
    }
}

struct Role<'a> {
    company: &'a str,
    link: Option<RoleLink<'a>>,
    title: &'a str,
    date_from: &'a str,
    date_to: &'a str,
    highlights: &'a [&'a str],
}

const ROLES: &'static [Role<'static>] = &[
    Role {
        company: "Fio's Quest",
        link: Some(RoleLink{ title: "fios-quest.com", href: "https://fios-quest.com"}),
        title: "Founder",
        date_from: "Jan '24",
        date_to: "Today",
        highlights: &[
            "Grew a YouTube channel to over 500 subscribers in just 8 videos",
            "Generated 12 chapters of innovative learning materials to teach the Rust programming language in simple steps",
            "Incorporated a company to manage revenue",
            "Built the website using Dioxus, a state of the art frontend framework of which Daniel is an early adopter",
            "Self taught videography and editing using Davinci Resolve",
        ],
    },
    Role {
        company: "Beamery",
        link: None,
        title: "Principal Engineer",
        date_from: "Nov '22",
        date_to: "Dec '23",
        highlights: &[
            "Mentored and Coached engineers at all levels",
            "Established a working group of 5 principal engineers to manage alignment across multiple teams",
            "Authored a comprehensive review of 5 IDaaS providers before architecting a passwordless auth system",
            "Lead a team of four engineers to build the candidate auth system",
            "Joined the leadership teams for Beamery’s Frontend and Backend Tribes",
            "Presented three “lunch and learns” on various topics (available on danielmason.com)",
            "Helped team go from 50% test coverage to 80% reducing out of hours outages to zero and improving morale",
        ],
    },
    Role {
        company: "Peloton",
        link: None,
        title: "Engineering Lead / Engineering Manager",
        date_from: "Sept '20",
        date_to: "Nov '22",
        highlights: &[
            "Lead the internationalisation team, then influenced the wider business to change practices to not require a specialised team, pivoting my team to content",
            "Grew the team from two engineers to eight",
            "Developed a new way to store and render content that was adopted by other teams, reducing the effort to produce a new page from two weeks and two engineers, to half a day and zero engineers",
            "Optimised project management to  improve reliability of estimations, reduced whole team meetings by 75% and increased team happiness",
            "Stayed on top of reports’ personal goals to ensure they achieved them and helped them progress their careers",
            "Cultivated an environment of psychological safety and prioritised team growth",
            "Successfully coached the manager who replaced me",
        ],
    },
    Role {
        company: "Triptease Ltd",
        link: None,
        title: "Senior Software Engineer",
        date_from: "Oct '19",
        date_to: "Sept '20",
        highlights: &[
            "Immediately resolved a bug that caused one of their systems to be unavailable for 45mins every day",
            "Formally career coached a data scientist and informally coached several engineers",
            "Gave multiple “lunch and learns” (the most popular of which covered the impact of thirsty Victorians on modern cryptography)",
            "Created a novel resolution for an advertising bug mitigating losses on ad spend of ~12%",
        ],
    },
    Role {
        company: "Apolitical Group Ltd",
        link: None,
        title: "Senior Software Engineer",
        date_from: "Nov '17",
        date_to: "Oct '19",
        highlights: &[
            "Planned and developed the platform architecture that took a service struggling service to over",
            "38x the number of users it had previously supported",
            "Coached two junior engineers",
            "Designed, documented and deployed the Continuous Deployment processes",
        ],
    },
    Role {
        company: "MOO Print Ltd",
        link: None,
        title: "Software Engineer",
        date_from: "May '16",
        date_to: "Nov '17",
        highlights: &[
            "Successfully advocated for and coached a junior engineer at risk of redundancy so that they could join our team",
            "Took a greenfield project to production",
            "Evangelized Continuous Deployment, participated in a working group and influenced business wide decisions",
        ],
    },
];

#[component]
pub fn Experience() -> Element {
    let roles = ROLES.iter().map(|role| {
        rsx!(
            div {
                h3 {
                    em {{role.company}}
                    " - "
                    {role.title}
                    {role.link.map(|link| link.render())}
                    br {}
                    {role.date_from}
                    " - "
                    {role.date_to}
                }
                ul {
                    for highlight in role.highlights {
                        li {{highlight}}
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
