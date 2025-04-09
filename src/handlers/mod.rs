use dioxus::prelude::*;

pub mod storage;

#[component]
pub fn ResourceHandler<T: Clone + 'static + std::cmp::PartialEq>(
    resource: Option<Option<T>>, 
    on_data: fn(T) -> Element
) -> Element {
    rsx! {
        match resource {
            Some(Some(data)) => on_data(data),
            Some(None) => rsx! {
                p { "Ошибка загрузки данных" }
            },
            None => rsx! {
                p { "Загрузка..." }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dioxus_ssr::{render, render_element};

    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    struct TestResponse {
        author: String,
        date: String,
    }

    #[test]
    fn resource_handler_with_data() {
        let data = TestResponse {
            author: String::from("Ouki"), 
            date: String::from("Date of publication"), 
        };

        let mut vdom = VirtualDom::new_with_props(move || {
            rsx! {
                ResourceHandler::<TestResponse> { 
                    resource: Some(Some(data.clone())),
                    on_data: move |wrapper| {
                        rsx! {
                            p { 
                                "{wrapper.author}"
                            }
                            p { 
                                "{wrapper.date}" 
                            }
                        }
                    }
                }
            }
        }, ());

        vdom.rebuild_in_place();

        assert_eq!(render(&vdom), render_element(rsx! {
            p { "Ouki" }
            p { "Date of publication" }
        }));
    }

    #[test]
    fn resource_handler_without_data() {
        let mut vdom = VirtualDom::new_with_props(move || {
            rsx! {
                ResourceHandler::<TestResponse> { 
                    resource: Some(None),
                    on_data: move |_| {
                        rsx!()
                    }
                }
            }
        }, ());

        vdom.rebuild_in_place();

        assert_eq!(render(&vdom), render_element(rsx! {
            p { "Ошибка загрузки данных" }
        }));
    }

    #[test]
    fn resource_handler_loading() {
        let mut vdom = VirtualDom::new_with_props(|| {
            rsx! {
                ResourceHandler::<TestResponse> { 
                    resource: None::<Option<TestResponse>>,
                    on_data: move |_| {
                        rsx!()
                    }
                }
            }
        }, ());

        vdom.rebuild_in_place();

        assert_eq!(render(&vdom), render_element(rsx! {
            p { "Загрузка..." }
        }));
    }
}
