#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

fn visualizer(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());
    cx.render(rsx! {
        div {
            class: "flex space-x-3",
            p {
                class: "float-left",
                "vaddr:"
            }

            form {
                onsubmit: |_| {},
                input {
                    value: 0,
                    oninput: move |event| {
                        match event.value.parse::<u64>() {
                            Ok(v) => name.set(format!("{v:032b}")),
                            Err(_e) => (),
                        }
                    }
                }
            }
        }

        div {
            class: "mx-auto p-8 flex justify-center",
            "{name}"
        }
    })
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "flex-grow bg-gray-300",
            div {
                class: "mx-auto p-8 text-3xl flex justify-center",
                "RISC-V address translation visualizer"
            }

            visualizer(cx)
        }

        footer {
            class: "info text-white bg-blue-900 text-center",
            p { "Copyright 2023 n.takana All rights reserved."}
            p { "github: ", a { href: "https://github.com/Alignof/SvxxVisualizer", "https://github.com/Alignof/SvxxVisualizer" }}
        }
    })
}
