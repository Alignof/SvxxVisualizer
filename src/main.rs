#![allow(non_snake_case)]
mod bit_field;

use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

fn visualizer(cx: Scope) -> Element {
    let vaddr = use_state(cx, || 0);
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
                    class: "bg-gray-900",
                    oninput: move |event|
                    if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                        if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                            vaddr.set(hex);
                        }
                    } else if let Ok(dec) = event.value.parse::<u64>() {
                        vaddr.set(dec);
                    }
                }
            }
        }

        bit_field::vaddr(cx, vaddr)
    })
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-700 text-white",
            main {
                div {
                    class: "p-8 text-3xl flex justify-center",
                    "RISC-V address translation visualizer"
                }

                visualizer(cx)
            }

            footer {
                class: "info text-white bg-blue-900 text-center mt-auto",
                p { "Copyright 2023 n.takana All rights reserved."}
                p { "github: ", a { href: "https://github.com/Alignof/SvxxVisualizer", "https://github.com/Alignof/SvxxVisualizer" }}
            }
        }
    })
}
