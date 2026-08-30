//! The signed-in guest, remembered in this browser.
//!
//! Booking still needs no account. This exists only for what the API will not
//! do anonymously — leaving a review and reading back your own — so the session
//! is deliberately thin: a token, a display name, nothing else.
//!
//! Same shape as [`crate::store`]: every function is a no-op on the server, so
//! SSR renders the signed-out view and hydration fills in the real one.

use crate::api::GuestSession;
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
const KEY: &str = "addis.guest";

#[cfg(feature = "hydrate")]
fn storage() -> Option<web_sys::Storage> {
    // Private mode and blocked site data throw rather than return null.
    leptos::prelude::window().local_storage().ok()?
}

/// The stored session, if this browser has one.
pub fn load() -> Option<GuestSession> {
    #[cfg(feature = "hydrate")]
    {
        let raw = storage().and_then(|s| s.get_item(KEY).ok().flatten())?;
        serde_json::from_str::<GuestSession>(&raw)
            .ok()
            .filter(|s| !s.access.is_empty())
    }
    #[cfg(not(feature = "hydrate"))]
    {
        None
    }
}

pub fn save(session: &GuestSession) {
    #[cfg(feature = "hydrate")]
    {
        if let (Some(s), Ok(raw)) = (storage(), serde_json::to_string(session)) {
            let _ = s.set_item(KEY, &raw);
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = session;
    }
}

pub fn clear() {
    #[cfg(feature = "hydrate")]
    {
        if let Some(s) = storage() {
            let _ = s.remove_item(KEY);
        }
    }
}

/// Bearer token for an authenticated call, or `None` when signed out.
pub fn token() -> Option<String> {
    load().map(|s| s.access)
}

/// Reactive session shared by the header and the pages that need it.
///
/// Provided once at the app root so signing in or out updates every consumer
/// without a reload. Reads `None` during SSR and picks up the stored session on
/// hydration.
#[derive(Copy, Clone)]
pub struct GuestCtx(pub RwSignal<Option<GuestSession>>);

/// Installs the context. Call once, inside the router.
pub fn provide_guest() -> GuestCtx {
    let ctx = GuestCtx(RwSignal::new(None));
    provide_context(ctx);
    // Effects never run on the server, so this is the hydration hand-off.
    Effect::new(move |_| ctx.0.set(load()));
    ctx
}

/// The session signal. Falls back to an empty signal if the context is missing,
/// so a component used outside the provider still renders.
pub fn use_guest() -> RwSignal<Option<GuestSession>> {
    use_context::<GuestCtx>()
        .map(|c| c.0)
        .unwrap_or_else(|| RwSignal::new(None))
}

/// Signs in: persists the session and updates every consumer.
pub fn sign_in(session: GuestSession) {
    save(&session);
    use_guest().set(Some(session));
}

/// Signs out.
pub fn sign_out() {
    clear();
    use_guest().set(None);
}
