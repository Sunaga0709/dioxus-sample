#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

// Run with hot reload: dx serve --hot-reload

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        header {
            class: "bg-green-400",
            "--- header area ---",
        },


        main {
            table {
                class: "border-separate",
                tr{
                    class: "bg-red-400",
                    th{
                        "id",
                    },
                    th{
                        "title",
                    },
                    th {
                        "description",
                    },
                    th{
                        "created_at",
                    },
                }
               for i in 1..=10 {
                    tr{
                        td {
                            "{i}"
                        },
                        td{
                            "title{i}",
                        },
                        td{
                            "description{i}",
                        },
                        td{
                            "2024-08-08 12:34:56",
                        }
                    }
                },
            },
        },

        footer {
            class: "bg-blue-400",
            "--- footer area ---",
        },
        // initial data
        // link { rel: "stylesheet", href: "main.css" }
        // img { src: "header.svg", id: "header" }
        // div { id: "links",
        //     a { target: "_blank", href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
        //     a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
        //     a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
        //     a { target: "_blank", href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
        //     a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
        //     a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        // }
    }
}
