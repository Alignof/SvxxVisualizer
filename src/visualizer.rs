use crate::config::Config;
use dioxus::prelude::*;

mod pte;
mod vaddr;

/// Global state
#[derive(Clone)]
struct TranslateState {
    /// Virtual page number.
    vpn: [u32; 3],
    /// Physical page number.
    ppn: [u32; 3],
    /// Showing each translate level flag.
    level_flags: [bool; 3],
    /// Showing result flag.
    result_flag: bool,
}

impl TranslateState {
    /// Constructor.
    pub fn new() -> Self {
        TranslateState {
            vpn: [0, 0, 0],
            ppn: [0, 0, 0],
            level_flags: [false, false, true],
            result_flag: false,
        }
    }

    /// Set VPN from the vpn_value.
    pub fn set_vpn(&mut self, vpn_value: u64) {
        self.vpn[0] = (vpn_value >> 12 & 0x1ff) as u32;
        self.vpn[1] = (vpn_value >> 21 & 0x1ff) as u32;
        self.vpn[2] = (vpn_value >> 30 & 0x1ff) as u32;
    }

    /// Return VPN value according to index.
    pub fn vpn(&self, index: usize) -> u64 {
        u64::from(self.vpn[index])
    }

    /// Set PPN from the ppn_value.
    pub fn set_ppn(&mut self, ppn_value: u64) {
        self.ppn[0] = (ppn_value >> 10 & 0x1ff) as u32;
        self.ppn[1] = (ppn_value >> 19 & 0x1ff) as u32;
        self.ppn[2] = (ppn_value >> 28 & 0x03ff_ffff) as u32;
    }

    /// Return PPN value according to index.
    pub fn ppn(&self, index: usize) -> u64 {
        u64::from(self.ppn[index])
    }

    /// Enable display flags according to pte value.
    pub fn enable_flags(&mut self, index: usize, pte: u64) {
        self.level_flags[index] = pte >> 1 & 0x1 == 0 && pte >> 3 & 0x1 == 0;
        self.result_flag = pte >> 1 & 0x1 == 1 || pte >> 3 & 0x1 == 1;
    }

    /// Return level_flags.
    pub fn flags(&self, index: usize) -> bool {
        self.level_flags[index]
    }

    /// Set result_flag.
    pub fn set_result_flag(&mut self, new_val: bool) {
        self.result_flag = new_val;
    }

    /// Return result_flag.
    pub fn result_flag(&self) -> bool {
        self.result_flag
    }

    /// Get current tranlate level (0 ~ 2).
    pub fn get_level(&self) -> usize {
        self.level_flags.iter().position(|x| *x).unwrap()
    }
}

fn trans_lv1<'a>(
    cx: Scope<'a>,
    pte_addr_1: &'a UseState<u64>,
    pte_1: &'a UseState<u64>,
    pte_addr_2: &'a UseState<u64>,
    trans_state: &'a UseState<TranslateState>,
) -> Element<'a> {
    let conf = use_shared_state::<Config>(cx).unwrap();
    let ppn =
        trans_state.get().ppn(2) << 18 | trans_state.get().ppn(1) << 9 | trans_state.get().ppn(0);
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            pte::pte_addr(cx, conf.read().satp_ppn, trans_state.get().vpn(2))

            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    format!("{:#x} (lv1 pte addr):", pte_addr_1.get())
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-900",
                        oninput: move |event|
                        if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                                pte_1.set(hex);
                                trans_state.with_mut(|t| {
                                    t.set_ppn(hex);
                                    t.enable_flags(1, hex);
                                });
                                pte_addr_2.set(ppn * 4096 + trans_state.get().vpn(1) * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte_1.set(dec);
                            trans_state.with_mut(|t| {
                                t.set_ppn(dec);
                                t.enable_flags(1, dec);
                            });
                            pte_addr_2.set(ppn * 4096 + trans_state.get().vpn(1) * 8);
                        }
                    }
                }
            }

            pte::bit_field(cx, pte_1)
            pte::pte_data(cx, pte_1)
        }
    })
}

fn trans_lv2<'a>(
    cx: Scope<'a>,
    pte_1: &'a UseState<u64>,
    pte_addr_2: &'a UseState<u64>,
    pte_2: &'a UseState<u64>,
    pte_addr_3: &'a UseState<u64>,
    trans_state: &'a UseState<TranslateState>,
) -> Element<'a> {
    let ppn =
        trans_state.get().ppn(2) << 18 | trans_state.get().ppn(1) << 9 | trans_state.get().ppn(0);
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            pte::pte_addr(cx, pte_1.get() >> 10 & 0x0fff_ffff_ffff, trans_state.get().vpn(1))

            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    format!("{:#x} (lv2 pte addr):", pte_addr_2.get())
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-900",
                        oninput: move |event|
                        if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                                pte_2.set(hex);
                                trans_state.with_mut(|t| {
                                    t.set_ppn(hex);
                                    t.enable_flags(0, hex);
                                });
                                pte_addr_3.set(ppn * 4096 + trans_state.get().vpn(0) * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte_2.set(dec);
                            trans_state.with_mut(|t| {
                                t.set_ppn(dec);
                                t.enable_flags(0, dec);
                            });
                            pte_addr_3.set(ppn * 4096 + trans_state.get().vpn(0) * 8);
                        }
                    }
                }
            }

            pte::bit_field(cx, pte_2)
            pte::pte_data(cx, pte_2)
        }
    })
}

fn trans_lv3<'a>(
    cx: Scope<'a>,
    pte_2: &'a UseState<u64>,
    pte_addr_3: &'a UseState<u64>,
    trans_state: &'a UseState<TranslateState>,
) -> Element<'a> {
    let pte_3 = use_state(cx, || 0);
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            pte::pte_addr(cx, pte_2.get() >> 10 & 0x0fff_ffff_ffff, trans_state.get().vpn(1))

            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    format!("{:#x} (lv2 pte addr):", pte_addr_3.get())
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-900",
                        oninput: move |event|
                        if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                                pte_3.set(hex);
                                trans_state.with_mut(|t| {
                                    t.set_ppn(hex);
                                    t.enable_flags(0, hex);
                                });
                                trans_state.with_mut(|t| t.set_result_flag(true));
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte_3.set(dec);
                            trans_state.with_mut(|t| {
                                t.set_ppn(dec);
                                t.enable_flags(0, dec);
                            });
                            trans_state.with_mut(|t| t.set_result_flag(true));
                        }
                    }
                }
            }

            pte::bit_field(cx, pte_2)
            pte::pte_data(cx, pte_2)
        }
    })
}

fn show_paddr<'a>(
    cx: Scope<'a>,
    page_off: u64,
    vaddr: &'a UseState<u64>,
    trans_state: &'a UseState<TranslateState>,
) -> Element<'a> {
    let level = trans_state.get().get_level();
    let trans = trans_state.get();
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            div {
                class: "p-4 text-xl bg-red-400",
                p {
                    format!("vaddr: {:#x}", vaddr.get())
                }
                p {
                    match level {
                        0 => format!("→ paddr: {:#x}", trans.ppn(2) << 30 | trans.ppn(1) << 21 | trans.ppn(0) << 12 | page_off),
                        1 => {
                            if trans.ppn(0) != 0 {
                                "trans.ppn(0) != 0".to_string()
                            } else {
                                format!("→ paddr: {:#x}", trans.ppn(2) << 30 | trans.ppn(1) << 21 | trans.vpn(0) << 12 | page_off)
                            }
                        }
                        2 => {
                            if trans.ppn(0) != 0 || trans.ppn(1) != 0 {
                                "trans.ppn(0) != 0 || trans.ppn(1) != 0".to_string()
                            } else {
                                format!("→ paddr: {:#x}", trans.ppn(2) << 30 | trans.vpn(1) << 21 | trans.vpn(0) << 12 | page_off)
                            }
                        }
                        _ => String::new()
                    }
                }
            }
        }
    })
}

/// Show address translation visualizer.
pub fn visualizer(cx: Scope) -> Element {
    let conf = use_shared_state::<Config>(cx).unwrap();
    let vaddr = use_state(cx, || 0);
    let trans_state = use_state(cx, TranslateState::new);
    let pte_addr_1 = use_state(cx, || 0);
    let pte_1 = use_state(cx, || 0);
    let pte_addr_2 = use_state(cx, || 0);
    let pte_2 = use_state(cx, || 0);
    let pte_addr_3 = use_state(cx, || 0);

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
                                trans_state.with_mut(|t| t.set_vpn(hex));
                                pte_addr_1.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(2) * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            vaddr.set(dec);
                            trans_state.with_mut(|t| t.set_vpn(dec));
                            pte_addr_1.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(2) * 8);
                        }
                    }
                }
            }

            vaddr::bit_field(cx, vaddr)
            vaddr::bit_data(cx, vaddr)
        }

        if trans_state.get().flags(2) {
            trans_lv1(cx, pte_addr_1, pte_1, pte_addr_2, trans_state)
        }

        if trans_state.get().flags(1) {
            trans_lv2(cx, pte_1, pte_addr_2, pte_2, pte_addr_3, trans_state)
        }

        if trans_state.get().flags(0) {
            trans_lv3(cx, pte_2, pte_addr_3, trans_state)
        }

        if trans_state.get().result_flag() {
            show_paddr(cx, vaddr.get() & 0xfff, vaddr, trans_state)
        }
    })
}
