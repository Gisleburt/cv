#![allow(non_snake_case)]

mod components;
mod content;
mod transforms;

use dioxus::prelude::*;

use components::*;
use content::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

const RESET_CSS: Asset = asset!("/assets/reset.css");
const SITE_CSS: Asset = asset!("/assets/site.css");

#[component]
fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: RESET_CSS }
        document::Link { rel: "stylesheet", href: SITE_CSS }


        Header { name: "Daniel Mason", title: "Engineering Lead" }

        main {
            div {

                PersonalStatement {}

                Skills {}

                ContactDetails {}

                Education {}
            }

            div { Experience {} }
        }
    }
}
