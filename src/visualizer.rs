use crate::config::Config;
use dioxus::prelude::*;

mod pte;
mod vaddr;

const MAX_LEVEL: usize = 3;

/// Global state
#[derive(Clone)]
struct TranslateState {
    /// Virtual address.
    vaddr: u64,

    /// Virtual page number.
    vpn: [u32; MAX_LEVEL],

    /// Page Table Entry each levels.
    pte_lv: [u64; MAX_LEVEL],

    /// Page Table Entry address each levels.
    pte_addr_lv: [u64; MAX_LEVEL],

    /// Current level.
    /// None -> has not tranlated yet.
    /// Some(x) -> level x (0~2).
    current_level: usize,

    /// Showing result flag.
    showing_result_flag: bool,
}

impl TranslateState {
    /// Constructor.
    pub fn new() -> Self {
        TranslateState {
            vaddr: 0,
            vpn: [0, 0, 0],
            pte_lv: [0, 0, 0],
            pte_addr_lv: [0, 0, 0],
            current_level: 1,
            showing_result_flag: false,
        }
    }

    /// Set vaddr.
    pub fn set_vaddr(&mut self, vaddr: u64) {
        self.vaddr = vaddr;
    }

    /// Get vaddr.
    pub fn get_vaddr(&self) -> u64 {
        self.vaddr
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

    /// Get PTE according to level.
    /// Level's range is 1~3 (convert to 0~2).
    pub fn get_pte(&self, level: usize) -> u64 {
        self.pte_lv[level - 1]
    }

    /// Set PTE according to level.
    /// Level's range is 1~3 (convert to 0~2).
    pub fn set_pte(&mut self, level: usize, pte_value: u64) {
        self.pte_lv[level - 1] = pte_value;
    }

    /// Get PTE address according to level.
    /// Level's range is 1~3 (convert to 0~2).
    pub fn get_pte_addr(&self, level: usize) -> u64 {
        self.pte_addr_lv[level - 1]
    }

    /// Set PTE address according to level.
    /// Level's range is 1~3 (convert to 0~2).
    pub fn set_pte_addr(&mut self, level: usize, pte_addr: u64) {
        self.pte_addr_lv[level - 1] = pte_addr;
    }

    /// Return PPN value according to index.
    pub fn ppn(&self, index: usize) -> u64 {
        let pte = self.pte_lv[self.current_level];
        match index {
            0 => pte >> 10 & 0x1ff,
            1 => pte >> 19 & 0x1ff,
            2 => pte >> 28 & 0x3ff_ffff,
            _ => unreachable!(),
        }
    }

    /// Enable display flags according to pte value.
    /// Level's range is 1~3 (convert to 0~2).
    pub fn update_level(&mut self, level: usize, pte: u64) {
        if pte >> 1 & 0x1 == 0 && pte >> 3 & 0x1 == 0 {
            self.current_level = level + 1;
        }
        self.showing_result_flag = pte >> 1 & 0x1 == 1 || pte >> 3 & 0x1 == 1;
    }

    /// Return showing_result_flag.
    pub fn result_flag(&self) -> bool {
        self.showing_result_flag
    }

    /// Get current tranlate level (0 ~ 2).
    pub fn get_level(&self) -> usize {
        self.current_level
    }
}

fn trans_each_level<'a>(
    cx: Scope<'a>,
    level: usize,
    trans_state: &'a UseState<TranslateState>,
) -> Element<'a> {
    let conf = use_shared_state::<Config>(cx).unwrap();
    let ppn =
        trans_state.get().ppn(2) << 18 | trans_state.get().ppn(1) << 9 | trans_state.get().ppn(0);
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            pte::pte_addr(cx, conf.read().satp_ppn, trans_state.get().vpn(MAX_LEVEL - level))

            div {
                class: "flex space-x-3 py-2",
                p {
                    class: "float-left text-lg",
                    format!("{:#x} (lv{level} pte addr):", trans_state.get_pte_addr(level))
                }

                form {
                    onsubmit: |_| {},
                    input {
                        class: "bg-gray-900",
                        oninput: move |event|
                        if let Some(hex_noprefix) = event.value.strip_prefix("0x") {
                            if let Ok(hex) = u64::from_str_radix(hex_noprefix, 16) {
                                trans_state.with_mut(|t| {
                                    t.set_pte(level, hex);
                                    t.update_level(level, hex);
                                    if level < MAX_LEVEL {
                                        t.set_pte_addr(level + 1, ppn * 4096 + trans_state.get().vpn(MAX_LEVEL - 1 - level) * 8);
                                    }
                                });
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            trans_state.with_mut(|t| {
                                t.set_pte(level, dec);
                                t.update_level(level, dec);
                                if level < MAX_LEVEL {
                                    t.set_pte_addr(level + 1, ppn * 4096 + trans_state.get().vpn(MAX_LEVEL - 1 - level) * 8);
                                }
                            });
                        }
                    }
                }
            }

            pte::bit_field(cx, level, trans_state)
            pte::pte_data(cx, level, trans_state)
        }
    })
}

fn show_paddr<'a>(cx: Scope<'a>, trans_state: &'a UseState<TranslateState>) -> Element<'a> {
    let page_off = trans_state.get_vaddr() & 0xfff;
    let level = trans_state.get().get_level();
    let trans = trans_state.get();
    cx.render(rsx! {
        div {
            class: "mx-auto p-8 flex flex-col justify-start",

            div {
                class: "p-4 text-xl bg-red-400",
                p {
                    format!("vaddr: {:#x}", trans_state.get_vaddr())
                }
                p {
                    match level + 1 {
                        1 => format!("→ paddr: {:#x}", trans.vpn(2) << 30 | trans.ppn(1) << 21 | trans.ppn(0) << 12 | page_off),
                        2 => {
                            if trans.ppn(0) != 0 {
                                "trans.ppn(0) != 0".to_string()
                            } else {
                                format!("→ paddr: {:#x}", trans.vpn(2) << 30 | trans.vpn(1) << 21 | trans.ppn(0) << 12 | page_off)
                            }
                        }
                        3 => {
                            if trans.ppn(0) != 0 || trans.ppn(1) != 0 {
                                "trans.ppn(0) != 0 || trans.ppn(1) != 0".to_string()
                            } else {
                                format!("→ paddr: {:#x}", trans.vpn(2) << 30 | trans.vpn(1) << 21 | trans.vpn(0) << 12 | page_off)
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
    let trans_state = use_state(cx, TranslateState::new);

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
                                trans_state.with_mut(|t| t.set_vaddr(hex));
                                trans_state.with_mut(|t| t.set_vpn(hex));
                                trans_state.with_mut(|t| t.set_pte_addr(1, conf.read().satp_ppn * 4096 + trans_state.get().vpn(2) * 8));
                            }
                        } else if let Ok(dec) = event.value.parse::<u64>() {
                            trans_state.with_mut(|t| t.set_vaddr(dec));
                            trans_state.with_mut(|t| t.set_vpn(dec));
                            trans_state.with_mut(|t| t.set_pte_addr(1, conf.read().satp_ppn * 4096 + trans_state.get().vpn(2) * 8));
                        }
                    }
                }
            }

            vaddr::bit_field(cx, trans_state)
            vaddr::bit_data(cx, trans_state)
        }

        p {
            format!("current level: {}", trans_state.get_level())
        }

        for level in 1..=MAX_LEVEL {
            if level <= trans_state.get_level() {
                trans_each_level(cx, level, trans_state)
            }
        }

        if trans_state.get().result_flag() {
            show_paddr(cx, trans_state)
        }
    })
}
