use crate::config::Config;
use dioxus::prelude::*;

mod pte;
mod vaddr;

pub fn visualizer(cx: Scope) -> Element {
    let conf = use_shared_state::<Config>(cx).unwrap();
    let vaddr = use_state(cx, || 0);
    let vpn_2 = use_state(cx, || 0);
    let pte_addr_1 = use_state(cx, || 0);
    let pte = use_state(cx, || 0);
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
                                vpn_2.set(hex >> 30 & 0x1ff);
                                pte_addr_1.set(conf.read().satp_ppn as u64 * 4096 + vpn_2.get() * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            vaddr.set(dec);
                            vpn_2.set(dec >> 30 & 0x1ff);
                            pte_addr_1.set(conf.read().satp_ppn as u64 * 4096 + vpn_2.get() * 8);
                        }
                    }
                }
            }

            vaddr::bit_field(cx, vaddr)
            vaddr::bit_data(cx, vaddr)
        }

        div {
            class: "mx-auto px-8 flex flex-col justify-start",

            pte::pte_addr(cx, conf.read().satp_ppn, *vpn_2.get())

            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    format!("{:#x} (pte addr):", pte_addr_1.get())
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-900",
                        oninput: move |event|
                        if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                                pte.set(hex)
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte.set(dec)
                        }
                    }
                }
            }

            pte::bit_field(cx, pte)
            pte::pte_data(cx, pte)
        }
    })
}
