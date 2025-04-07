use dioxus::prelude::*;
use web_sys::{
    wasm_bindgen::JsCast, 
    window, 
    HtmlElement
};

mod app;
pub use app::*;

mod setup;
pub use setup::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/rasp")]
        Rasp {},
    #[end_layout]

    #[nest("/setup")]
    #[layout(SetupLayout)]
        #[route("/group")]
        GroupSelector {},
    #[end_layout]
    #[end_nest]

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>
    }
}

#[component]
fn AppLayout() -> Element {
    use_effect(move || {
        if let Some(document) = window().and_then(|w| w.document()) {
            let navbar = document.get_element_by_id("navbar");
            let content = document.get_element_by_id("content");

            if let (Some(navbar), Some(content)) = (navbar, content) {
                let navbar_height = navbar
                    .get_bounding_client_rect()
                    .height();

                content
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .style()
                    .set_property("padding-bottom", 
                        &format!("calc({}px + var(--padding) * 4)", navbar_height))
                    .unwrap();
            }
        }
    });

    rsx! {
        div {
            id: "content",
            Outlet::<Route> {}
        }
        nav {
            id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Rasp {}, "Rasp" }
        }
    }
}

#[component]
fn SetupLayout() -> Element {
    rsx! {
        div {
            id: "content",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Страница не найдена" }
        p { "Запрашиваемый ресурс: {route:?}" }
    }
}
