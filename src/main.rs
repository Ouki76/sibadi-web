use dioxus::prelude::*;

mod route;
mod handlers;
mod types;

const FAVICON: Asset = asset!("/assets/favicon.ico");

const MAIN_CSS: Asset = asset!("/assets/main.css");
const VARS_CSS: Asset = asset!("/assets/vars.css");
const WIDGETS_CSS: Asset = asset!("/assets/widgets.css");

fn main() {
    dioxus::launch(|| rsx! {
        // Favicon
        document::Link { rel: "icon", href: FAVICON }
        // Css
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: VARS_CSS }
        document::Link { rel: "stylesheet", href: WIDGETS_CSS }

        // Draw route
        Router::<route::Route> {}
    });
}
