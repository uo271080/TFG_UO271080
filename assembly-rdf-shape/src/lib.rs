#![recursion_limit = "512"]

mod app;
mod components;
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}

#[test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

