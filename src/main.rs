#![allow(non_snake_case)]

mod components;
mod content;

use dioxus::prelude::*;
use tracing::Level;

use components::*;
use content::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

#[component]
fn Home() -> Element {
    rsx! {
        Header { name: "Daniel Mason", title: "Engineering Lead" }

        div {
            class: "main-grid",

            div {

                PersonalStatement {}

                Skills {}

                ContactDetails {}

                Education {}
            }

            div {

                Experience {}
            }

        }



    }
}
