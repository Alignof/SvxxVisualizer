use dioxus::prelude::*;

pub fn vaddr<'a>(cx: Scope<'a>, vaddr: &'a UseState<u64>) -> Element<'a> {
    let vaddr_bytes = vaddr.get().to_le_bytes();
    let boxes = vaddr_bytes
        .iter()
        .rev()
        .map(|byte| {
            let upper = byte >> 4;
            let lower = byte & 0xf;
            let bit_box = |bit: u8| {
                rsx! {
                    div {
                        class: "flex flex-col items-center",
                        div {
                            class: "p-2 text-xl border-2 bg-slate-600 font-mono",
                            "{bit:04b}"
                        }
                        div {
                            class: "text-xl",
                            "{bit:x}"
                        }
                    }
                }
            };
            rsx! {
                bit_box(upper)
                bit_box(lower)
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
