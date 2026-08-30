//! One reservation, as the guest sees it.
//!
//! Read from [`crate::store`] — the browser's cache of bookings it has legitimately
//! held, filled in when the booking was made or retrieved with a one-time code.
//! The hotel's own details are fetched live so the address and phone number are
//! current even for a booking cached weeks ago.

use crate::api::{get_hotel_detail, money_round, pretty_date, HotelDetail, Reservation};
use crate::components::{pluralize, use_toast, Icon};
use crate::store;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};

fn status_class(status: &str) -> &'static str {
    match status {
        "Confirmed" | "In-house" | "Completed" => "bg-emerald-100 text-emerald-700",
        "Cancelled" | "Rejected" | "No show" => "bg-red-100 text-red-700",
        _ => "bg-amber-100 text-amber-700",
    }
}

#[component]
pub fn ReservationDetailsPage() -> impl IntoView {
    let params = use_params_map();
    let reference = move || params.get().get("booking_ref").unwrap_or_default();

    let booking = RwSignal::new(Option::<Reservation>::None);
    let loaded = RwSignal::new(false);
    Effect::new(move |_| {
        booking.set(store::get(&reference()));
        loaded.set(true);
    });

    let hotel = Resource::new(
        move || booking.get().map(|b| b.organization).unwrap_or(0),
        |id| async move {
            if id == 0 {
                return Err(ServerFnError::new("no hotel"));
            }
            get_hotel_detail(id.to_string()).await
        },
    );

    view! {
        <Title text=move || format!("Reservation {} — Horn of Africa Hotel Portal", reference()) />

        <div class="mx-auto max-w-3xl px-4 py-8">
            <A href="/my-reservations" attr:class="group mb-4 inline-flex items-center gap-1.5 text-sm font-semibold text-slate-500 transition-colors hover:text-blue-700">
                <Icon name="chevron-left" class="h-4 w-4 transition-transform duration-200 group-hover:-translate-x-0.5" />
                "My reservations"
            </A>

            {move || match (loaded.get(), booking.get()) {
                (false, _) => view! { <div class="skeleton h-72 rounded-2xl"></div> }.into_any(),
                (true, None) => view! { <NeedsRetrieval reference=reference() /> }.into_any(),
                (true, Some(b)) => view! { <Details booking=b hotel=hotel /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn Details(
    booking: Reservation,
    hotel: Resource<Result<HotelDetail, ServerFnError>>,
) -> impl IntoView {
    let toast = use_toast();
    let navigate = use_navigate();
    let nav = StoredValue::new(navigate);

    let subtotal = booking.room_subtotal();
    let ref_for_forget = StoredValue::new(booking.reference());
    // The Suspense body re-renders, so everything it reads is stored, not moved.
    let reference = StoredValue::new(booking.reference());
    let status = StoredValue::new(booking.status().to_string());
    let held = StoredValue::new(booking);

    let forget = move |_| {
        if !crate::confirm(
            "Remove this booking from this browser? The reservation itself is unaffected — you can retrieve it again with a code.",
        ) {
            return;
        }
        store::forget(&ref_for_forget.get_value());
        toast.info("Removed", "This browser no longer lists that booking.");
        nav.with_value(|n| n("/my-reservations", Default::default()));
    };

    view! {
        <Suspense fallback=|| view! { <div class="skeleton h-72 rounded-2xl"></div> }>
            {move || Suspend::new(async move {
                let b = held.get_value();
                let reference = reference.get_value();
                let status = status.get_value();
                let h = hotel.await.ok();
                let policies = h
                    .as_ref()
                    .and_then(|h| h.policies.clone())
                    .unwrap_or_default();
                let hotel_name = h
                    .as_ref()
                    .map(|h| h.name.clone())
                    .unwrap_or_else(|| b.organization_name.clone());
                let location = h.as_ref().map(|h| h.location()).unwrap_or_default();
                let phone = h.as_ref().and_then(|h| h.phone_number.clone());
                let email = h.as_ref().and_then(|h| h.email.clone());
                let currency = h
                    .as_ref()
                    .map(|h| h.currency_code().to_string())
                    .unwrap_or_else(|| "ETB".into());
                let hotel_href = h.as_ref().map(|h| format!("/hotels/{}", h.id));
                let map_url = format!(
                    "https://www.google.com/maps/search/?api=1&query={}",
                    location.replace(' ', "+")
                );

                view! {
                    // ---- Header ---------------------------------------
                    <div class="rounded-2xl border border-slate-200 bg-white p-6">
                        <div class="flex flex-wrap items-start justify-between gap-3">
                            <div class="min-w-0">
                                <p class="text-xs font-semibold uppercase tracking-[0.2em] text-slate-400">
                                    "Booking reference"
                                </p>
                                <h1 class="mt-1 break-all text-2xl font-bold tracking-wide tabular-nums text-ink">
                                    {reference.clone()}
                                </h1>
                            </div>
                            <span class=format!(
                                "rounded-full px-3 py-1 text-xs font-bold {}",
                                status_class(&status)
                            )>
                                {status.clone()}
                            </span>
                        </div>

                        <div class="mt-5 border-t border-slate-100 pt-5">
                            <p class="text-lg font-bold text-ink">{hotel_name.clone()}</p>
                            {(!location.is_empty()).then(|| view! {
                                <p class="mt-1 flex flex-wrap items-center gap-1.5 text-sm text-slate-500">
                                    <Icon name="map-pin" class="h-3.5 w-3.5 shrink-0" />
                                    {location.clone()}
                                    <a href=map_url target="_blank" rel="noopener noreferrer" class="font-semibold text-blue-700 hover:underline">
                                        "Map"
                                    </a>
                                </p>
                            })}
                            <p class="mt-1 text-sm text-slate-500">
                                {format!("{} · Room {}", b.room.name.clone(), b.room.room_number.clone())}
                            </p>
                        </div>

                        <dl class="mt-5 grid grid-cols-2 gap-4 border-t border-slate-100 pt-5 sm:grid-cols-4">
                            <Fact icon="calendar" label="Check-in"
                                value=pretty_date(b.check_in_date.as_deref())
                                hint=format!("from {}", policies.checkin_display()) />
                            <Fact icon="calendar" label="Check-out"
                                value=pretty_date(b.check_out_date.as_deref())
                                hint=format!("by {}", policies.checkout_display()) />
                            <Fact icon="moon" label="Nights"
                                value=b.number_of_nights.to_string() hint="1 room" />
                            <Fact icon="users" label="Guests"
                                value=pluralize(b.guest_count, "guest")
                                hint={
                                    if b.extra_bed > 0 {
                                        pluralize(b.extra_bed, "extra bed")
                                    } else {
                                        "No extra beds".to_string()
                                    }
                                } />
                        </dl>

                        <div class="mt-5 flex flex-wrap items-end justify-between gap-3 rounded-xl bg-slate-50 px-4 py-3.5">
                            <span>
                                <span class="block text-sm font-bold text-slate-800">"Room charge"</span>
                                <span class="block text-xs text-slate-500">"Payable at the hotel · taxes added at checkout"</span>
                            </span>
                            <span class="text-xl font-bold tabular-nums text-ink">
                                {format!("{currency} {}", money_round(subtotal))}
                            </span>
                        </div>
                    </div>

                    // ---- Guest ----------------------------------------
                    <div class="mt-5 rounded-2xl border border-slate-200 bg-white p-6">
                        <h2 class="text-sm font-bold text-ink">"Guest details"</h2>
                        <dl class="mt-3 flex flex-col gap-2.5 text-sm">
                            <Row label="Name" value=b.guest.name.clone() />
                            <Row label="Email" value=b.guest.email.clone().unwrap_or_default() />
                            <Row label="Phone" value=b.guest.phone_number.clone().unwrap_or_default() />
                            <Row label="Booked" value=pretty_date(b.created_at.as_deref()) />
                        </dl>
                        {b.guest_note.clone().filter(|n| !n.trim().is_empty()).map(|note| view! {
                            <div class="mt-3 rounded-xl bg-blue-50 px-3 py-2.5 text-sm text-blue-900">
                                <p class="text-xs font-bold uppercase tracking-wide text-blue-700">"Your request"</p>
                                <p class="mt-0.5">{note}</p>
                            </div>
                        })}
                        {b.cancellation_reason.clone().filter(|n| !n.trim().is_empty()).map(|reason| view! {
                            <div class="mt-3 rounded-xl bg-red-50 px-3 py-2.5 text-sm text-red-900">
                                <p class="text-xs font-bold uppercase tracking-wide text-red-700">"Reason"</p>
                                <p class="mt-0.5">{reason}</p>
                            </div>
                        })}
                    </div>

                    // ---- Contact the hotel ----------------------------
                    <div class="mt-5 rounded-2xl border border-slate-200 bg-white p-6">
                        <h2 class="text-sm font-bold text-ink">"Need to change something?"</h2>
                        <p class="mt-1 text-sm text-slate-500">
                            "Changes and cancellations are handled by the property directly — contact them with your reference."
                        </p>
                        <div class="mt-3 flex flex-wrap gap-2">
                            {phone.filter(|p| !p.is_empty()).map(|p| {
                                let href = format!("tel:{}", p.replace(' ', ""));
                                view! {
                                    <a href=href class="flex items-center gap-1.5 rounded-xl bg-blue-700 px-4 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800">
                                        <Icon name="phone" class="h-4 w-4" />
                                        {p}
                                    </a>
                                }
                            })}
                            {email.filter(|e| !e.is_empty()).map(|e| {
                                let href = format!("mailto:{e}?subject=Booking%20{}", reference.clone());
                                view! {
                                    <a href=href class="flex items-center gap-1.5 rounded-xl border border-slate-300 px-4 py-2.5 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50">
                                        <Icon name="mail" class="h-4 w-4" />
                                        "Email the hotel"
                                    </a>
                                }
                            })}
                            {hotel_href.map(|href| view! {
                                <A href=href attr:class="flex items-center gap-1.5 rounded-xl border border-slate-300 px-4 py-2.5 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50">
                                    <Icon name="building" class="h-4 w-4" />
                                    "Hotel page"
                                </A>
                            })}
                        </div>
                    </div>

                    <button
                        on:click=forget
                        class="mt-5 w-full rounded-xl border border-slate-300 py-2.5 text-sm font-semibold text-slate-600 transition-colors hover:bg-slate-50"
                    >
                        "Remove from this browser"
                    </button>
                }.into_any()
            })}
        </Suspense>
    }
}

/// Shown when this browser has never held the booking.
#[component]
fn NeedsRetrieval(#[prop(into)] reference: String) -> impl IntoView {
    view! {
        <div class="rounded-2xl border border-slate-200 bg-white p-8 text-center">
            <span class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                <Icon name="lock" class="h-6 w-6" />
            </span>
            <h1 class="text-xl font-bold text-ink">"Verify to see this booking"</h1>
            <p class="mx-auto mt-2 max-w-sm text-sm leading-relaxed text-slate-500">
                {format!(
                    "Reservations are private. To open {reference} we'll send a one-time code to the email or phone recorded on it.",
                )}
            </p>
            <A
                href="/retrieve-booking"
                attr:class="mt-5 inline-flex items-center gap-1.5 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
            >
                <Icon name="key" class="h-4 w-4" />
                "Retrieve with a code"
            </A>
        </div>
    }
}

#[component]
fn Fact(
    icon: &'static str,
    label: &'static str,
    #[prop(into)] value: String,
    #[prop(into)] hint: String,
) -> impl IntoView {
    view! {
        <div>
            <dt class="flex items-center gap-1 text-[11px] uppercase tracking-wide text-slate-400">
                <Icon name=icon class="h-3 w-3" />
                {label}
            </dt>
            <dd class="mt-0.5 text-sm font-bold text-slate-800">{value}</dd>
            <dd class="text-[11px] text-slate-400">{hint}</dd>
        </div>
    }
}

#[component]
fn Row(label: &'static str, #[prop(into)] value: String) -> impl IntoView {
    let value = if value.trim().is_empty() {
        "—".to_string()
    } else {
        value
    };
    view! {
        <div class="flex items-start justify-between gap-3">
            <dt class="shrink-0 text-slate-500">{label}</dt>
            <dd class="min-w-0 break-words text-right font-semibold text-slate-800">{value}</dd>
        </div>
    }
}
