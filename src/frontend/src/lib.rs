#![recursion_limit = "8192"]

/// Wrap static class string in closure to avoid compile-time type recursion.
/// Leptos normalizes static classes into nested types; wrapping in a closure
/// defers to runtime and breaks the type chain.
#[macro_export]
macro_rules! cls {
    ($classes:expr) => {
        ($classes, || true)
    };
}

pub mod app;
pub mod pages;
pub use components;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
