//! Bookings this browser knows about.
//!
//! The portal has no accounts, and the API has no "list my reservations"
//! endpoint — a booking is read back only with a one-time code. So this page
//! lists what [`crate::store`] cached when the booking was made or retrieved,
//! and points anyone else at the retrieval flow.

use crate::api::{money_round, pretty_date, Reservation};
use crate::components::{pluralize, Icon};
use crate::store;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const TABS: &[&str] = &["Upcoming", "Past", "Cancelled"];

fn status_class(status: &str) -> &'static str {
    match status {
        "Confirmed" | "In-house" | "Completed" => "bg-emerald-100 text-emerald-700",
        "Cancelled" | "Rejected" | "No show" => "bg-red-100 text-red-700",
        _ => "bg-amber-100 text-amber-700",
    }
}

/// Which tab a booking belongs in.
fn bucket(r: &Reservation) -> &'static str {
    match r.status() {
        "Cancelled" | "Rejected" | "No show" => "Cancelled",
        "Completed" => "Past",
        _ => "Upcoming",
    }
}

#[component]
pub fn MyReservationsPage() -> impl IntoView {
    let tab = RwSignal::new(TABS[0]);
    let bookings = RwSignal::new(Vec::<Reservation>::new());
    let loaded = RwSignal::new(false);

    // `store` is client-side only, so this fills in on hydration.
    Effect::new(move |_| {
        bookings.set(store::all());
        loaded.set(true);
    });

    let visible = move || {
        let t = tab.get();
        bookings
            .get()
            .into_iter()
            .filter(|r| bucket(r) == t)
            .collect::<Vec<_>>()
    };
    let count_in = move |t: &'static str| bookings.get().iter().filter(|r| bucket(r) == t).count();

    view! {
        <Title text="My reservations — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-3xl px-4 py-8">
            <div class="flex flex-wrap items-end justify-between gap-3">
                <div>
                    <h1 class="text-2xl font-extrabold tracking-tight text-slate-900">"My reservations"</h1>
                    <p class="mt-1 text-sm text-slate-500">
                        "Bookings made or retrieved in this browser."
                    </p>
                </div>
                <A
                    href="/retrieve-booking"
                    attr:class="flex items-center gap-1.5 rounded-xl border border-slate-300 px-4 py-2.5 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50"
                >
                    <Icon name="key" class="h-4 w-4" />
                    "Add a booking"
                </A>
            </div>

            <div class="mt-5 flex flex-wrap gap-2">
                {TABS.iter().map(|t| {
                    let t = *t;
                    view! {
                        <button
                            class=move || format!(
                                "rounded-xl px-4 py-2 text-sm font-semibold transition-colors {}",
                                if tab.get() == t { "bg-blue-700 text-white" } else { "border border-slate-300 bg-white text-slate-600 hover:bg-slate-50" }
                            )
                            on:click=move |_| tab.set(t)
                        >
                            {t}
                            <Show when=move || (count_in(t) > 0)>
                                <span class=move || format!(
                                    "ml-1.5 rounded-full px-1.5 py-0.5 text-[10px] font-bold {}",
                                    if tab.get() == t { "bg-white/20" } else { "bg-slate-100 text-slate-500" }
                                )>
                                    {move || count_in(t)}
                                </span>
                            </Show>
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="mt-5">
                {move || {
                    if !loaded.get() {
                        return view! { <div class="skeleton h-40 rounded-2xl"></div> }.into_any();
                    }
                    if bookings.get().is_empty() {
                        return view! { <NothingSaved/> }.into_any();
                    }
                    let rows = visible();
                    if rows.is_empty() {
                        return view! {
                            <p class="rounded-2xl border border-slate-200 bg-white px-4 py-10 text-center text-sm text-slate-500">
                                {format!("No {} bookings in this browser.", tab.get().to_lowercase())}
                            </p>
                        }.into_any();
                    }
                    view! {
                        <div class="flex flex-col gap-3">
                            {rows.into_iter().map(|r| view! { <BookingCard booking=r /> }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>

            <p class="mt-6 rounded-2xl bg-slate-100 px-4 py-3 text-xs leading-relaxed text-slate-500">
                <span class="font-bold text-slate-700">"Why only this browser? "</span>
                "There are no accounts here — a booking is proven by the email or phone on it. Clearing your site data hides these, but the reservations themselves are safe: retrieve any of them again with your reference and a one-time code."
            </p>
        </div>
    }
}

#[component]
fn BookingCard(booking: Reservation) -> impl IntoView {
    let reference = booking.reference();
    let status = booking.status().to_string();
    let href = format!("/reservation/{reference}");

    view! {
        <A href=href attr:class="card-hover block rounded-2xl border border-slate-200 bg-white p-4">
            <div class="flex flex-wrap items-start justify-between gap-3">
                <div class="min-w-0">
                    <p class="flex flex-wrap items-center gap-2">
                        <span class="font-bold text-slate-900">{booking.organization_name.clone()}</span>
                        <span class=format!(
                            "rounded-full px-2 py-0.5 text-[11px] font-bold {}",
                            status_class(&status)
                        )>
                            {status.clone()}
                        </span>
                    </p>
                    <p class="mt-0.5 text-xs text-slate-500">
                        {format!("{} · Room {}", booking.room.name.clone(), booking.room.room_number.clone())}
                    </p>
                    <p class="mt-1.5 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-slate-500">
                        <span class="flex items-center gap-1">
                            <Icon name="calendar" class="h-3.5 w-3.5" />
                            {format!(
                                "{} → {}",
                                pretty_date(booking.check_in_date.as_deref()),
                                pretty_date(booking.check_out_date.as_deref()),
                            )}
                        </span>
                        <span class="flex items-center gap-1">
                            <Icon name="moon" class="h-3.5 w-3.5" />
                            {pluralize(booking.number_of_nights, "night")}
                        </span>
                        <span class="flex items-center gap-1">
                            <Icon name="users" class="h-3.5 w-3.5" />
                            {pluralize(booking.guest_count, "guest")}
                        </span>
                    </p>
                </div>

                <div class="shrink-0 text-right">
                    <p class="text-[11px] uppercase tracking-wide text-slate-400">{reference.clone()}</p>
                    <p class="mt-0.5 text-lg font-extrabold tabular-nums text-slate-900">
                        {format!("ETB {}", money_round(booking.room_subtotal()))}
                    </p>
                    <p class="text-[11px] text-slate-400">"at the hotel"</p>
                </div>
            </div>
        </A>
    }
}

#[component]
fn NothingSaved() -> impl IntoView {
    view! {
        <div class="rounded-2xl border border-slate-200 bg-white px-6 py-12 text-center">
            <span class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                <Icon name="calendar" class="h-6 w-6" />
            </span>
            <h2 class="text-lg font-bold text-slate-900">"No bookings in this browser yet"</h2>
            <p class="mx-auto mt-2 max-w-sm text-sm leading-relaxed text-slate-500">
                "Book a room and it appears here automatically. Already booked elsewhere? Retrieve it with your reference and a one-time code."
            </p>
            <div class="mt-5 flex flex-col justify-center gap-3 sm:flex-row">
                <A href="/hotels" attr:class="rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800">
                    "Browse hotels"
                </A>
                <A href="/retrieve-booking" attr:class="rounded-xl border border-slate-300 px-5 py-2.5 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50">
                    "Retrieve a booking"
                </A>
            </div>
        </div>
    }
}
