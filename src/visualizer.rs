use crate::config::Config;
use dioxus::prelude::*;

mod bit_field;

pub fn visualizer(cx: Scope) -> Element {
    let conf = use_shared_state::<Config>(cx).unwrap();
    let vaddr = use_state(cx, || 0);
    let pte_addr = use_state(cx, || 0);
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",
            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
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
                                pte_addr.set(conf.read().satp_ppn as u64 * 4096 + 0 * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            vaddr.set(dec);
                            pte_addr.set(conf.read().satp_ppn as u64 * 4096 + 0 * 8);
                        }
                    }
                }
            }

            bit_field::vaddr(cx, vaddr)
        }

        div {
            class: "mx-auto px-8 flex flex-col justify-start",
            div {
                p {
                    class: "float-left text-xl",
                    "pte addr = PPN × PAGESIZE + VPN[2] × pte_size"
                }
            }

            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    format!("{:#x} (pte addr):", pte_addr.get())
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-900",
                        oninput: move |event|
                        if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                                //vaddr.set(hex);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            //vaddr.set(dec);
                        }
                    }
                }
            }
        }
    })
}
