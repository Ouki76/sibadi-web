use dioxus::prelude::*;

#[component]
pub fn Rasp() -> Element {
    rsx! {
        for i in 0..201 {
            p {
                "{i}"
            }
        }
    }
}
