use dioxus::prelude::*;

enum VaddrField {
    Poff,
    Vpn0,
    Vpn1,
    Vpn2,
    Resv,
}

use VaddrField::*;
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
                class: "p-2 text-xl border-2 bg-slate-600 font-mono",
                div {
                    for i in 0..4 {
                        span {
                            class: match color_map[i] {
                                Poff => "text-red-600",
                                Vpn0 => "text-green-600",
                                Vpn1 => "text-blue-300",
                                Vpn2 => "text-yellow-600",
                                Resv => "text-white-600",
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

pub fn vaddr<'a>(cx: Scope<'a>, vaddr: &'a UseState<u64>) -> Element<'a> {
    let vaddr_bytes = vaddr.get().to_le_bytes();
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
            class: "mx-auto p-8 flex justify-center",
            for bf in boxes {
                bf
            }
        }
    })
}
