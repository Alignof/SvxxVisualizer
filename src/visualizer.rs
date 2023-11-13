use crate::config::Config;
use dioxus::prelude::*;

mod pte;
mod vaddr;

#[derive(Clone)]
struct TranslateState {
    vpn: [u32; 3],
    ppn: [u32; 3],
    level_flags: [bool; 3],
    result_flag: bool,
}

impl TranslateState {
    pub fn new() -> Self {
        TranslateState {
            vpn: [0, 0, 0],
            ppn: [0, 0, 0],
            level_flags: [false, false, true],
            result_flag: false,
        }
    }

    pub fn init_vpn(&mut self, vpn_value: u64) {
        self.vpn[0] = vpn_value as u32 >> 12 & 0x1ff;
        self.vpn[1] = vpn_value as u32 >> 21 & 0x1ff;
        self.vpn[2] = vpn_value as u32 >> 30 & 0x1ff;
    }

    pub fn vpn(&self, index: usize) -> u64 {
        self.vpn[index] as u64
    }

    pub fn set_ppn(&mut self, ppn_value: u64) {
        self.ppn[0] = ppn_value as u32 >> 10 & 0x1ff;
        self.ppn[1] = ppn_value as u32 >> 19 & 0x1ff;
        self.ppn[2] = ppn_value as u32 >> 28 & 0x3ffffff;
    }

    pub fn enable_flags(&mut self, index: usize, pte: u64) {
        self.level_flags[index] = pte >> 1 & 0x1 == 0 && pte >> 3 & 0x1 == 0;
    }

    pub fn flags(&self, index: usize) -> bool {
        self.level_flags[index]
    }

    pub fn result_flag(&self) -> bool {
        self.result_flag
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
                                pte_addr_2.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(1) * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte_1.set(dec);
                            trans_state.with_mut(|t| {
                                t.set_ppn(dec);
                                t.enable_flags(1, dec);
                            });
                            pte_addr_2.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(1) * 8);
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
    let conf = use_shared_state::<Config>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            pte::pte_addr(cx, pte_1.get() >> 10 & 0xfffffffffff, trans_state.get().vpn(1))

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
                                pte_addr_3.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(0) * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte_2.set(dec);
                            trans_state.with_mut(|t| {
                                t.set_ppn(dec);
                                t.enable_flags(0, dec);
                            });
                            pte_addr_3.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(0) * 8);
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

            pte::pte_addr(cx, pte_2.get() >> 10 & 0xfffffffffff, trans_state.get().vpn(1))

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
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            pte_3.set(dec);
                            trans_state.with_mut(|t| {
                                t.set_ppn(dec);
                                t.enable_flags(0, dec);
                            });
                        }
                    }
                }
            }

            pte::bit_field(cx, pte_2)
            pte::pte_data(cx, pte_2)
        }
    })
}

pub fn visualizer(cx: Scope) -> Element {
    let conf = use_shared_state::<Config>(cx).unwrap();
    let vaddr = use_state(cx, || 0);
    let trans_state = use_state(cx, || TranslateState::new());
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
                                trans_state.with_mut(|t| t.init_vpn(hex));
                                pte_addr_1.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(2) * 8);
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            vaddr.set(dec);
                            trans_state.with_mut(|t| t.init_vpn(dec));
                            pte_addr_1.set(conf.read().satp_ppn * 4096 + trans_state.get().vpn(2) * 8);
                        }
                    }
                }
            }

            vaddr::bit_field(cx, vaddr)
            vaddr::bit_data(cx, vaddr)
        }

        if trans_state.get().flags(2) {
            trans_lv1(cx, &pte_addr_1, &pte_1, &pte_addr_2, &trans_state)
        }

        if trans_state.get().flags(1) {
            trans_lv2(cx, &pte_1, &pte_addr_2, &pte_2, &pte_addr_3, &trans_state)
        }

        if trans_state.get().flags(0) {
            trans_lv3(cx, &pte_2, &pte_addr_3, &trans_state)
        }
    })
}
