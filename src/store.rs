//! Bookings remembered in this browser.
//!
//! The portal deliberately has no accounts: a guest books with nothing but a
//! name and a way to reach them. The API reflects that — there is no "list my
//! reservations" endpoint, and reading a booking back requires a one-time code
//! sent to the email or phone on it.
//!
//! So "My reservations" means *the bookings made or retrieved in this browser*.
//! Each one is cached here at the moment we legitimately hold it: right after
//! booking, or right after an OTP retrieval. Everything is client-side; on the
//! server every function is a no-op so SSR renders the empty state and hydration
//! fills it in.

use crate::api::Reservation;

const KEY: &str = "addis.bookings";
/// Enough to be useful, small enough never to strain `localStorage`.
const LIMIT: usize = 20;

#[cfg(feature = "hydrate")]
fn storage() -> Option<web_sys::Storage> {
    // Private mode and blocked site data both make this throw rather than
    // return null, so every access is fallible.
    leptos::prelude::window().local_storage().ok()?
}

/// Every booking this browser knows about, most recent first.
pub fn all() -> Vec<Reservation> {
    #[cfg(feature = "hydrate")]
    {
        let Some(raw) = storage().and_then(|s| s.get_item(KEY).ok().flatten()) else {
            return Vec::new();
        };
        serde_json::from_str(&raw).unwrap_or_default()
    }
    #[cfg(not(feature = "hydrate"))]
    {
        Vec::new()
    }
}

/// One booking by reference, if this browser has seen it.
pub fn get(reference: &str) -> Option<Reservation> {
    all().into_iter().find(|r| r.reference() == reference)
}

/// Remembers a booking, replacing any earlier copy of the same reference so a
/// re-retrieval refreshes a status that has since changed.
pub fn remember(reservation: &Reservation) {
    #[cfg(feature = "hydrate")]
    {
        let reference = reservation.reference();
        if reference.is_empty() {
            return;
        }
        let mut list = all();
        list.retain(|r| r.reference() != reference);
        list.insert(0, reservation.clone());
        list.truncate(LIMIT);

        if let (Some(s), Ok(json)) = (storage(), serde_json::to_string(&list)) {
            let _ = s.set_item(KEY, &json);
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = reservation;
    }
}

/// Forgets one booking — the guest asked this browser to stop listing it. The
/// reservation itself is untouched; it can always be retrieved again with a code.
pub fn forget(reference: &str) {
    #[cfg(feature = "hydrate")]
    {
        let mut list = all();
        list.retain(|r| r.reference() != reference);
        if let (Some(s), Ok(json)) = (storage(), serde_json::to_string(&list)) {
            let _ = s.set_item(KEY, &json);
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = reference;
    }
}
