use dioxus::prelude::*;

use crate::{
    handlers::{storage::{self, Storage}, ResourceHandler},
    types::GroupsResponse
};

#[component]
pub fn GroupSelector() -> Element {
    let groups = use_resource(move || async move {
        let years_list = reqwest::get("https://umu.sibadi.org/api/Rasp/ListYears")
            .await
            .ok()?
            .json::<serde_json::Value>()
            .await
            .ok()?;

        let url = format!(
            "https://umu.sibadi.org/api/raspGrouplist?year={}",
            years_list["data"]["years"]
                .as_array()
                .unwrap_or(&Vec::new())
                .last()
                .unwrap_or(&serde_json::Value::default())
                .as_str()
                .unwrap_or_default()
        );

        let response = reqwest::get(url)
            .await
            .ok()?
            .json::<GroupsResponse>()
            .await
            .ok()?;

        Some(response)
    });

    rsx! {
        div { class: "header",
            h1 { "Добро пожаловать" }
        }
        div { class: "body",
            ResourceHandler::<GroupsResponse> { 
                resource: groups.read().clone(),
                on_data: move |wrapper| {
                    rsx! {
                        for group in wrapper.data {
                            button {
                                onclick: move |_| {
                                    storage::Local::set("group", &group.id.to_string());
                                },
                                "{group.name}"
                            }
                        }
                    }
                }
            }
        }
        div { class: "footer",
            button { "Click!" }
        }
    }
}
