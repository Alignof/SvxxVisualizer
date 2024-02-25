use crate::visualizer::TranslateState;
use dioxus::prelude::*;

enum VaddrField {
    Poff,
    Vpn0,
    Vpn1,
    Vpn2,
    Resv,
}

use VaddrField::{Poff, Resv, Vpn0, Vpn1, Vpn2};
const VADDR_COLOR_MAP: [[VaddrField; 8]; 8] = [
    [Resv, Resv, Resv, Resv, Resv, Resv, Resv, Resv],
    [Resv, Resv, Resv, Resv, Resv, Resv, Resv, Resv],
    [Resv, Resv, Resv, Resv, Resv, Resv, Resv, Resv],
    [Resv, Vpn2, Vpn2, Vpn2, Vpn2, Vpn2, Vpn2, Vpn2],
    [Vpn2, Vpn2, Vpn1, Vpn1, Vpn1, Vpn1, Vpn1, Vpn1],
    [Vpn1, Vpn1, Vpn1, Vpn0, Vpn0, Vpn0, Vpn0, Vpn0],
    [Vpn0, Vpn0, Vpn0, Vpn0, Poff, Poff, Poff, Poff],
    [Poff, Poff, Poff, Poff, Poff, Poff, Poff, Poff],
];

fn bit_box<'a>(cx: Scope<'a>, bit: u8, color_map: &[VaddrField]) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center",
            div {
                class: "p-2 text-xl border-2 font-mono",
                div {
                    for i in 0..4 {
                        span {
                            class: match color_map[i] {
                                Poff => "text-red-500",
                                Vpn0 => "text-green-500",
                                Vpn1 => "text-blue-300",
                                Vpn2 => "text-yellow-500",
                                Resv => "text-white-500",
                            },
                            format!("{:01b}", bit >> (3 - i) & 1),
                        }
                    }
                }
            }
            div {
                class: "text-xl",
                "{bit:x}"
            }
        }
    })
}

pub fn bit_field<'a>(cx: Scope<'a>, trans_state: &'a UseState<TranslateState>) -> Element<'a> {
    let vaddr_bytes = trans_state.get_vaddr().to_be_bytes();
    let boxes = vaddr_bytes
        .iter()
        .rev()
        .zip(VADDR_COLOR_MAP)
        .map(|(byte, cmap)| {
            let upper = byte >> 4;
            let lower = byte & 0xf;
            rsx! {
                bit_box(cx, upper, &cmap[0..4])
                bit_box(cx, lower, &cmap[4..8])
            }
        })
        .collect::<Vec<_>>();

    cx.render(rsx! {
        div {
            class: "flex justify-start",
            for bf in boxes {
                bf
            }
        }
    })
}

pub fn bit_data<'a>(cx: Scope<'a>, trans_state: &'a UseState<TranslateState>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "py-1 flex text-xl text-yellow-500 font-mono",
            div {
                "vpn[2]:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", trans_state.get_vaddr() >> 30 & 0x1ff)
            }
        }
        div {
            class: "py-1 flex text-xl text-blue-300 font-mono",
            div {
                "vpn[1]:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", trans_state.get_vaddr() >> 21 & 0x1ff)
            }
        }
        div {
            class: "py-1 flex text-xl text-green-500 font-mono",
            div {
                "vpn[0]:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", trans_state.get_vaddr() >> 12 & 0x1ff)
            }
        }
        div {
            class: "py-1 flex text-xl text-red-500 font-mono",
            div {
                "page offset:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", trans_state.get_vaddr() & 0xfff)
            }
        }
    })
}
