#![recursion_limit = "512"]

mod app;

use wasm_bindgen::prelude::*;
use yew::utils::document;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    // let onload_closure = Closure::wrap(Box::new(|| {
    //     init_yashe("yashe-editor");
    // }) as Box<dyn Fn()>);
    // document().set_onload(Some(onload_closure.as_ref().unchecked_ref()));
    // onload_closure.forget();
    Ok(())
}

// #[wasm_bindgen(module = "/src/yashe.js")]
// extern "C" {
//     #[wasm_bindgen(js_name = initYASHE)]
//     fn init_yashe(element_id: &str) -> JsValue;
// }

