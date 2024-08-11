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
       Header{disabled: false},


        main {
            class: "my-4 flex justify-center",
            Table{
                table_class: "table-auto border border-collapse border-gray-500",
                headers: vec!["id","title", "description", "created_at"],
                table_header_class: "border border-gray-700 bg-gray-300 px-2.5",
                values: vec![
                    vec![String::from("1"), String::from("title_1"), String::from("description_1"), String::from("2024-08-08 12:34:56.123456")],
                    vec![String::from("2"), String::from("title_2"), String::from("description_2"), String::from("2024-08-08 12:34:56.123456")],
                    vec![String::from("3"), String::from("title_3"), String::from("description_3"), String::from("2024-08-08 12:34:56.123456")],
                    vec![String::from("4"), String::from("title_4"), String::from("description_4"), String::from("2024-08-08 12:34:56.123456")],
                    vec![String::from("5"), String::from("title_5"), String::from("description_5"), String::from("2024-08-08 12:34:56.123456")],
                ],
                table_data_class: "border border-gray-700",
            },
        },

       Footer{disabled: false},
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
struct HeaderProps {
    disabled: bool,
}

#[component]
fn Header(props: HeaderProps) -> Element {
    if props.disabled {
        rsx! {}
    } else {
        rsx! {
            header{
                class: "bg-green-400 flex justify-center",
                "--- header area ---",
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
struct FooterProps {
    disabled: bool,
}

#[component]
fn Footer(props: FooterProps) -> Element {
    if props.disabled {
        rsx! {}
    } else {
        rsx! {
            footer{
                class: "bg-blue-400 flex justify-center",
                "--- footer area ---",
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
struct TableProps {
    table_class: &'static str,
    headers: Vec<&'static str>,
    table_header_class: &'static str,
    values: Vec<Vec<String>>,
    table_data_class: &'static str,
}

#[component]
fn Table(props: TableProps) -> Element {
    rsx! {
        table{
            class: "{props.table_class}",
            tr {
                class: "{props.table_header_class}",
                for header in props.headers {
                    th {
                        class: "{props.table_header_class}",
                        "{header}",
                    }
                }
            },
            for value in props.values {
                tr{
                    for item in value {
                        td{
                            class: "{props.table_data_class}",
                            "{item}",
                        }
                    }
                }
            }
        }
    }
}
