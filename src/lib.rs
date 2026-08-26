#![recursion_limit = "512"]
// Comparisons inside `view!` attributes must be parenthesised, otherwise the
// macro treats `>` as the end of the tag. rustc then sees the parens as
// redundant in the expanded code, so the lint is switched off crate-wide.
#![allow(unused_parens)]

pub mod app;
pub mod components;
pub mod data;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
