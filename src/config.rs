//! setting configulation about address translation.

use dioxus::prelude::*;

/// address translation config
pub struct Config {
    /// satp.ppn
    pub satp_ppn: u64,
}

impl Config {
    pub fn new() -> Self {
        Config { satp_ppn: 0 }
    }

    pub fn set_ppn(&mut self, new_val: &str) {
        if let Some(hex_noprefix) = new_val.strip_prefix("0x") {
            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                self.satp_ppn = hex;
            }
        } else if let Ok(dec) = new_val.parse::<u64>() {
            self.satp_ppn = dec;
        }
    }
}

pub fn config(cx: Scope<'_>) -> Element<'_> {
    let conf = use_shared_state::<Config>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start bg-gray-500",
            div {
                class: "text-2xl py-2",
                "--config--"
            }
            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    "satp.ppn:"
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-800",
                        oninput: move |event| conf.write().set_ppn(&event.value)
                    }
                }
            }
        }
    })
}
