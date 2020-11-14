mod pages;
mod components;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

use pages::Home;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}

// --
