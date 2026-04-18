/// Wrap static class string in closure to avoid compile-time type recursion.
/// Leptos normalizes static classes into nested types; wrapping in a closure
/// defers to runtime and breaks the type chain.
#[macro_export]
macro_rules! cls {
    ($classes:expr) => {
        ($classes, || true)
    };
}

pub mod custom;
pub mod hooks;
pub mod layout;
pub mod ui;
