pub mod route;
pub mod handlers;
pub mod types;

#[macro_export]
macro_rules! assert_rsx_eq {
    ($first:expr, $second:expr) => {
        pretty_assertions::assert_str_eq!(
            dioxus_ssr::render_element($first),
            dioxus_ssr::render_element($second)
        );
    };
}
