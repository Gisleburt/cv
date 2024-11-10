use crate::components::*;
use dioxus::prelude::*;

struct SkillList<'a>(&'a str, &'a [&'a str]);

#[component]
pub fn Skills() -> Element {
    let skills = [
        SkillList(
            "Leadership",
            &[
                "Engineering Manager",
                "Architect",
                "Tech Evangelist",
                "People Manager",
                "Coach",
                "Mentor",
                "Cross-Team Collaborator",
                "Stakeholder Manager",
            ],
        ),
        SkillList(
            "Languages",
            &["Rust", "TypeScript", "JavaScript", "SQL", "Node.js"],
        ),
        SkillList(
            "Quality Control",
            &[
                "GitLab CI",
                "GitHub Actions",
                "rustdoc",
                "Jest",
                "ESLint",
                "Prettier",
                "Mocha",
                "mdBook",
            ],
        ),
        SkillList("Methodology", &["Agile", "TDD", "BDD", "DDD", "CI/CD", "Scrum", "Kanban", "Scrumban"]),
        SkillList(
            "Frameworks",
            &["React", "Next.js", "Express", "Actix", "Dioxus"],
        ),
        SkillList(
            "Platforms",
            &["Kubernetes", "GKE", "GCP", "Docker", "AWS", "Linux"],
        ),
    ];

    let all_skills = skills.iter().map(|skill_list| {
        rsx!(
            dt { {skill_list.0} }
            dd { {skill_list.1.join(", ")} }
        )
    });

    rsx!(
        Section { title: "Skills at a glance",
            dl { {all_skills} }
        }
    )
}
