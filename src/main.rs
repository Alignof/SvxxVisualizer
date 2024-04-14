mod config;
mod visualizer;

use config::Config;
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

/// create a root component that renders a page.
fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, Config::new);
    cx.render(rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-700 text-white",
            main {
                div {
                    class: "p-8 text-3xl flex justify-center",
                    "RISC-V Address Translation Visualizer"
                }

                config::config(cx)

                visualizer::visualizer(cx)
            }

            footer {
                class: "info text-white bg-cyan-950 text-center mt-auto",
                p { "Copyright 2023-2024, n.takana All rights reserved."}
                p { "github: ", a { href: "https://github.com/Alignof/SvxxVisualizer", "https://github.com/Alignof/SvxxVisualizer" }}
            }
        }
    })
}
