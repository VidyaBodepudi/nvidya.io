mod app;
mod animation;
mod components;

use leptos::prelude::*;

fn main() {
    // Better panic messages in the browser console
    console_error_panic_hook::set_once();

    // Initialize logging
    let _ = console_log::init_with_level(log::Level::Debug);

    log::info!("nvidya.io WASM app initializing...");

    mount_to_body(app::App);
}
