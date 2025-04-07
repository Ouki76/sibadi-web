use dioxus::prelude::*;

mod rasp;
pub use rasp::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Home page" }
    }
}
