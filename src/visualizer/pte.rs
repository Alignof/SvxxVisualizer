use dioxus::prelude::*;

enum PteField {
    Flags,
    Rsw,
    Ppn0,
    Ppn1,
    Ppn2,
    Resv,
    PBMT,
    N,
}

use PteField::*;
const PTE_COLOR_MAP: [[PteField; 8]; 8] = [
    [N, PBMT, Resv, Resv, Resv, Resv, Resv, Resv],
    [Resv, Resv, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2],
    [Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2],
    [Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2, Ppn2],
    [Ppn2, Ppn2, Ppn2, Ppn2, Ppn1, Ppn1, Ppn1, Ppn1],
    [Ppn1, Ppn1, Ppn1, Ppn1, Ppn1, Ppn0, Ppn0, Ppn0],
    [Ppn0, Ppn0, Ppn0, Ppn0, Ppn0, Ppn0, Rsw, Rsw],
    [Flags, Flags, Flags, Flags, Flags, Flags, Flags, Flags],
];

fn bit_box<'a>(cx: Scope<'a>, bit: u8, color_map: &[PteField]) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center",
            div {
                class: "p-2 text-xl border-2 bg-slate-600 font-mono",
                div {
                    for i in 0..4 {
                        span {
                            class: match color_map[i] {
                                Flags => "text-red-500",
                                Rsw => "text-purple-400",
                                Ppn0 => "text-green-500",
                                Ppn1 => "text-blue-300",
                                Ppn2 => "text-yellow-500",
                                Resv | PBMT | N => "text-white-500",
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

pub fn bit_field<'a>(cx: Scope<'a>, pte: &'a UseState<u64>) -> Element<'a> {
    let pte_bytes = pte.get().to_le_bytes();
    let boxes = pte_bytes
        .iter()
        .rev()
        .zip(PTE_COLOR_MAP)
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

pub fn pte_data<'a>(cx: Scope<'a>, pte: &'a UseState<u64>) -> Element<'a> {
    let flag_name = ["V", "R", "W", "X", "U", "G", "A", "D"];
    cx.render(rsx! {
        div {
            class: "py-1 flex text-xl text-yellow-500 font-mono",
            div {
                "ppn[2]:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", pte.get() >> 28 & 0x3ffffff)
            }
        }
        div {
            class: "py-1 flex text-xl text-blue-300 font-mono",
            div {
                "ppn[1]:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", pte.get() >> 19 & 0x1ff)
            }
        }
        div {
            class: "py-1 flex text-xl text-green-500 font-mono",
            div {
                "ppn[0]:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", pte.get() >> 10 & 0x1ff)
            }
        }
        div {
            class: "py-1 flex text-xl text-purple-400 font-mono",
            div {
                "RSW:"
            }
            div {
                class: "pl-2 text-white",
                format!("{:#03x}", pte.get() >> 8 & 0x3)
            }
        }
        div {
            class: "py-1 flex text-xl text-red-500 font-mono",
            div {
                "flags(D,A,G,U,X,W,R,V):"
            }
            div {
                class: "pl-2 text-white",
                for i in 0..8 {
                    if pte.get() >> i & 0x1 == 1 {
                        format!("{} ", flag_name[i])
                    } else {
                        format!("")
                    }
                }
            }
        }
    })
}

pub fn pte_addr(cx: Scope, satp_ppn: u64, vpn: u64) -> Element {
    cx.render(rsx! {
        div {
            p {
                class: "float-left text-xl",
                format!(
                    "pte addr = {:#x} × {:#x} + {:#x} × {:#x}",
                    satp_ppn,
                    4096,
                    vpn,
                    8
                )
            }
        }
    })
}
