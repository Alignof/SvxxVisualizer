use dioxus::prelude::*;

mod bit_field;

pub fn visualizer(cx: Scope) -> Element {
    let vaddr = use_state(cx, || 0);
    cx.render(rsx! {
        div {
            class: "flex space-x-3 p-2",
            p {
                class: "float-left",
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
                        }
                    } else if let Ok(dec) = event.value.parse::<u64>() {
                        vaddr.set(dec);
                    }
                }
            }
        }

        bit_field::vaddr(cx, vaddr)
    })
}
