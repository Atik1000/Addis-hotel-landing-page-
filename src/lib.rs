#![recursion_limit = "512"]
// Comparisons inside `view!` attributes must be parenthesised, otherwise the
// macro treats `>` as the end of the tag. rustc then sees the parens as
// redundant in the expanded code, so the lint is switched off crate-wide.
#![allow(unused_parens)]

pub mod api;
pub mod app;
pub mod components;
pub mod data;
pub mod images;
pub mod pages;
pub mod store;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

/// Browser confirmation dialog, for actions the guest cannot undo. Returns
/// `false` on the server, where there is nobody to ask.
pub fn confirm(message: &str) -> bool {
    #[cfg(feature = "hydrate")]
    {
        return web_sys::window()
            .and_then(|w| w.confirm_with_message(message).ok())
            .unwrap_or(false);
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = message;
        false
    }
}
