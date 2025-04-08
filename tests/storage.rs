#![allow(dead_code)]

use wasm_bindgen_test::*;
use sibadi_web::handlers::storage::{Local, Session, Storage};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn session_storage() {
    Session::set("foo", "bar");
    assert_eq!(Session::get("foo"), Some("bar".to_string()));

    Session::remove("foo");
    assert_eq!(Session::get("foo"), None);
}

#[wasm_bindgen_test]
fn local_storage() {
    Local::set("foo", "bar");
    assert_eq!(Local::get("foo"), Some("bar".to_string()));

    Local::remove("foo");
    assert_eq!(Local::get("foo"), None);
}
