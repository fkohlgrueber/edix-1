#![recursion_limit="2048"]
mod app;
pub mod cursor;
pub mod content;
pub mod controller;
pub mod highlight;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();

    let mount_point = yew::utils::document().query_selector("#mount-point").unwrap().unwrap();
    
    yew::App::<app::App>::new().mount(mount_point);

    yew::run_loop();

    Ok(())
}
