mod api;
mod app;
mod page;
mod component;
mod route;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<app::App>();
}
