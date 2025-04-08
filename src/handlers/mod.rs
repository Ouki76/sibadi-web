use dioxus::prelude::*;

#[component]
pub fn ResourceHandler<T: Clone + 'static + std::cmp::PartialEq>(
    resource: Option<Option<T>>, 
    on_data: fn(T) -> Element) 
-> Element {
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
