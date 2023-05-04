#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod components;
mod errors;

pub use errors::AppError;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        head {
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@2.51.6/dist/full.css" }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2/dist/tailwind.min.css" }
            style { include_str!("./style.css") }
        }
        body { class: "bg-gray-200 flex content-center justify-center",
            "data-theme": "dark",
            div { class: "my-10 card w-96 bg-base-200 shadow-xl w-3/4 overflow-auto",
                components::secret_santa::secret_santa{}
            }
        }
    })
}
