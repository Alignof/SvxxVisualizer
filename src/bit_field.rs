use dioxus::prelude::*;

enum VaddrField {
    PageOff,
    Vpn0,
    Vpn1,
    Vpn2,
    Reserved,
}

use VaddrField::*;
const VADDR_COLOR_MAP: [[VaddrField; 8]; 8] = [
    [
        // lower ------> upper
        PageOff, PageOff, PageOff, PageOff, PageOff, PageOff, PageOff, PageOff,
    ],
    [
        PageOff, PageOff, PageOff, PageOff, PageOff, PageOff, PageOff, PageOff,
    ],
    [
        PageOff, PageOff, PageOff, PageOff, PageOff, PageOff, PageOff, PageOff,
    ],
    [Vpn0, Vpn0, Vpn0, Vpn0, Vpn0, Vpn0, Vpn0, Vpn0],
    [Vpn0, Vpn1, Vpn1, Vpn1, Vpn1, Vpn1, Vpn1, Vpn1],
    [Vpn1, Vpn1, Vpn2, Vpn2, Vpn2, Vpn2, Vpn2, Vpn2],
    [
        Vpn2, Vpn2, Vpn2, Reserved, Reserved, Reserved, Reserved, Reserved,
    ],
    [
        Reserved, Reserved, Reserved, Reserved, Reserved, Reserved, Reserved, Reserved,
    ],
];

fn bit_box<'a>(cx: Scope<'a>, bit: u8, color_map: &'a [VaddrField]) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center",
            div {
                class: "p-2 text-xl border-2 bg-slate-600 font-mono",
                for i in 0..4 {
                    p {
                        class: match color_map[i] {
                            PageOff => "text-red",
                            Vpn0 => "text-blue",
                            Vpn1 => "text-green",
                            Vpn2 => "text-megenta",
                            Reserved => "text-white",
                        },
                        format!("{:01b}", bit >> i & 1),
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
