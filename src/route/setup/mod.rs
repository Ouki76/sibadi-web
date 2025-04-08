use dioxus::prelude::*;

#[component]
pub fn GroupSelector() -> Element {
    rsx! {
        div { class: "header",
            h1 { "Добро пожаловать" }
        }
        div { class: "body",
            for i in 0..200 {
                p { "i: {i}" }
            }
        }
        div { class: "footer",
            button { "Click!" }
        }
    }
}
