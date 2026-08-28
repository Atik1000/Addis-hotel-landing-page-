//! The screen shown straight after a booking.
//!
//! The reservation is read from [`crate::store`] — the booking wizard caches it
//! there the moment the API returns it, because there is no public endpoint that
//! reads a booking back without a one-time code. Opening this URL in another
//! browser therefore shows the reference and a link to retrieve it properly,
//! rather than pretending to know details it cannot see.

use crate::api::{get_hotel_detail, money_round, pretty_date, HotelDetail, Reservation};
use crate::components::{use_toast, Icon};
use crate::store;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ConfirmationPage() -> impl IntoView {
    let params = use_params_map();
    let reference = move || params.get().get("booking_ref").unwrap_or_default();

    // `store` only has anything after hydration, so this is a signal rather than
    // a value read once during SSR.
    let booking = RwSignal::new(Option::<Reservation>::None);
    Effect::new(move |_| {
        booking.set(store::get(&reference()));
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
        <Title text=move || format!("Reservation {} confirmed — Horn of Africa Hotel Portal", reference()) />

        <div class="mx-auto max-w-2xl px-4 py-10">
            // ---- Hero ------------------------------------------------
            <div class="flex flex-col items-center text-center">
                <div class="relative mb-5 flex h-20 w-20 items-center justify-center">
                    <span class="absolute inset-0 animate-pulse-ring rounded-full bg-emerald-400"></span>
                    <span class="relative flex h-20 w-20 animate-scale-in items-center justify-center rounded-full bg-emerald-500 text-white shadow-xl shadow-emerald-500/30">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" class="h-9 w-9">
                            <path class="animate-draw-check" d="M5 12.5l4.5 4.5L19 7.5" />
                        </svg>
                    </span>
                </div>

                <h1 class="animate-fade-up text-3xl font-extrabold tracking-tight text-slate-900" style="animation-delay: 120ms">
                    "Reservation submitted"
                </h1>
                <p class="mt-2 max-w-md animate-fade-up text-sm leading-relaxed text-slate-500" style="animation-delay: 170ms">
                    "Your room is held while the hotel confirms it. Show the reference below at the front desk — nothing has been charged online."
                </p>
            </div>

            // ---- Reference ticket -------------------------------------
            <div class="relative mt-7 animate-fade-up overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-lg shadow-slate-900/5" style="animation-delay: 220ms">
                <div class="bg-gradient-to-br from-blue-700 via-blue-800 to-indigo-900 px-6 py-7 text-center text-white">
                    <p class="text-xs font-semibold uppercase tracking-[0.2em] text-blue-200">"Booking reference"</p>
                    <p class="mt-2 break-all text-4xl font-extrabold tracking-[0.14em] tabular-nums">{reference}</p>
                    <CopyButton reference=Signal::derive(reference) />
                </div>

                // Perforated edge between the header and the details.
                <div class="relative h-4 bg-white">
                    <span class="absolute -left-2 top-1/2 h-4 w-4 -translate-y-1/2 rounded-full bg-slate-50 ring-1 ring-slate-200"></span>
                    <span class="absolute -right-2 top-1/2 h-4 w-4 -translate-y-1/2 rounded-full bg-slate-50 ring-1 ring-slate-200"></span>
                    <span class="absolute inset-x-5 top-1/2 border-t-2 border-dashed border-slate-200"></span>
                </div>

                <div class="px-6 pb-6">
                    {move || match booking.get() {
                        Some(b) => view! { <BookingDetails booking=b hotel=hotel /> }.into_any(),
                        None => view! {
                            <div class="py-4 text-center">
                                <p class="text-sm text-slate-600">
                                    "This browser does not have the details for this booking."
                                </p>
                                <p class="mt-1 text-xs text-slate-400">
                                    "Bookings are private — retrieve it with the code we send to the email or phone on the reservation."
                                </p>
                                <A
                                    href="/retrieve-booking"
                                    attr:class="mt-4 inline-flex items-center gap-1.5 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                                >
                                    <Icon name="key" class="h-4 w-4" />
                                    "Retrieve this booking"
                                </A>
                            </div>
                        }.into_any(),
                    }}
                </div>
            </div>

            // ---- Next steps -------------------------------------------
            <div class="mt-6 animate-fade-up rounded-2xl border border-slate-200 bg-white p-5" style="animation-delay: 280ms">
                <h2 class="text-sm font-bold text-slate-900">"What happens next"</h2>
                <ol class="mt-3 flex flex-col gap-3">
                    <Step n="1" title="The hotel confirms" body="You will hear from the property directly if anything needs checking." />
                    <Step n="2" title="Keep your reference" body="It is how you retrieve the booking and how the front desk finds you." />
                    <Step n="3" title="Pay on arrival" body="Settle directly with the hotel. This portal never takes payment and charges no fee." />
                </ol>
            </div>

            // ---- Actions ----------------------------------------------
            <div class="mt-6 flex flex-col gap-3 sm:flex-row">
                <A
                    href=move || format!("/reservation/{}", reference())
                    attr:class="flex flex-1 items-center justify-center gap-1.5 rounded-xl bg-blue-700 px-5 py-3 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                >
                    "View reservation"
                    <Icon name="arrow-right" class="h-4 w-4" />
                </A>
                <A
                    href="/hotels"
                    attr:class="flex flex-1 items-center justify-center gap-1.5 rounded-xl border border-slate-300 px-5 py-3 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50"
                >
                    "Browse more hotels"
                </A>
            </div>
        </div>
    }
}

/// The stay itself, once this browser holds the booking.
#[component]
fn BookingDetails(
    booking: Reservation,
    hotel: Resource<Result<HotelDetail, ServerFnError>>,
) -> impl IntoView {
    let subtotal = booking.room_subtotal();
    let status = StoredValue::new(booking.status().to_string());
    let held = StoredValue::new(booking);

    view! {
        <Suspense fallback=|| view! { <div class="skeleton h-20 rounded-xl"></div> }>
            {move || Suspend::new(async move {
                let b = held.get_value();
                let status = status.get_value();
                let h = hotel.await.ok();
                let (name, location, phone, checkin, checkout) = match &h {
                    Some(h) => (
                        h.name.clone(),
                        h.location(),
                        h.phone_number.clone(),
                        h.policies.clone().unwrap_or_default().checkin_display(),
                        h.policies.clone().unwrap_or_default().checkout_display(),
                    ),
                    None => (b.organization_name.clone(), String::new(), None, "—".into(), "—".into()),
                };
                let logo = h
                    .as_ref()
                    .and_then(|h| h.gallery().first().cloned());
                let currency = h
                    .as_ref()
                    .map(|h| h.currency_code().to_string())
                    .unwrap_or_else(|| "ETB".into());

                view! {
                    <div class="flex gap-4">
                        {match logo {
                            Some(src) => view! {
                                <img src=src alt=name.clone() class="h-20 w-20 shrink-0 rounded-xl object-cover" />
                            }.into_any(),
                            None => view! {
                                <div class="flex h-20 w-20 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-blue-600 to-indigo-700 text-2xl font-extrabold text-white">
                                    {name.chars().next().unwrap_or('H').to_uppercase().to_string()}
                                </div>
                            }.into_any(),
                        }}
                        <div class="min-w-0">
                            <p class="text-base font-bold text-slate-900">{name.clone()}</p>
                            <Show when={ let l = location.clone(); move || !l.is_empty() }>
                                <p class="flex items-center gap-1 text-xs text-slate-500">
                                    <Icon name="map-pin" class="h-3 w-3" />
                                    {location.clone()}
                                </p>
                            </Show>
                            <p class="mt-1 text-xs text-slate-500">
                                {format!(
                                    "{} · Room {}",
                                    b.room.name.clone(),
                                    b.room.room_number.clone(),
                                )}
                            </p>
                            {phone.filter(|p| !p.is_empty()).map(|p| {
                                let href = format!("tel:{}", p.replace(' ', ""));
                                view! {
                                    <a href=href class="mt-1.5 inline-flex items-center gap-1 text-xs font-bold text-blue-700 hover:underline">
                                        <Icon name="phone" class="h-3 w-3" />
                                        {p}
                                    </a>
                                }
                            })}
                        </div>
                    </div>

                    <dl class="mt-5 grid grid-cols-2 gap-3 border-t border-slate-100 pt-5 text-sm sm:grid-cols-4">
                        <Fact icon="calendar" label="Check-in"
                            value=pretty_date(b.check_in_date.as_deref()) hint=checkin />
                        <Fact icon="calendar" label="Check-out"
                            value=pretty_date(b.check_out_date.as_deref()) hint=checkout />
                        <Fact icon="moon" label="Nights"
                            value=b.number_of_nights.to_string() hint="1 room" />
                        <Fact icon="users" label="Guests"
                            value=b.guest_count.to_string() hint="Adults" />
                    </dl>

                    <div class="mt-5 flex flex-wrap items-end justify-between gap-3 rounded-xl bg-slate-50 px-4 py-3.5">
                        <span>
                            <span class="block text-sm font-bold text-slate-800">"Room charge"</span>
                            <span class="block text-xs text-slate-500">
                                "Payable at the hotel · taxes added at checkout"
                            </span>
                        </span>
                        <span class="text-xl font-extrabold tabular-nums text-slate-900">
                            {format!("{currency} {}", money_round(subtotal))}
                        </span>
                    </div>

                    <div class="mt-3 flex flex-wrap items-center gap-2">
                        <span class=format!(
                            "rounded-full px-2.5 py-1 text-xs font-bold {}",
                            match status.as_str() {
                                "Confirmed" | "In-house" | "Completed" => "bg-emerald-100 text-emerald-700",
                                "Cancelled" | "Rejected" | "No show" => "bg-red-100 text-red-700",
                                _ => "bg-amber-100 text-amber-700",
                            }
                        )>
                            {status.clone()}
                        </span>
                        <span class="text-xs text-slate-400">
                            {format!("Booked {}", pretty_date(b.created_at.as_deref()))}
                        </span>
                    </div>

                    {b.guest_note.clone().filter(|n| !n.trim().is_empty()).map(|note| view! {
                        <div class="mt-3 rounded-xl bg-blue-50 px-3 py-2 text-xs text-blue-900">
                            <span class="font-bold">"Your request: "</span>
                            {note}
                        </div>
                    })}
                }.into_any()
            })}
        </Suspense>
    }
}

/// Copies the reference to the clipboard, falling back to a reminder toast when
/// the browser refuses.
#[component]
fn CopyButton(reference: Signal<String>) -> impl IntoView {
    let toast = use_toast();
    let copied = RwSignal::new(false);

    let copy = move |_| {
        let value = reference.get();
        #[cfg(feature = "hydrate")]
        {
            let clipboard = window().navigator().clipboard();
            let _ = clipboard.write_text(&value);
        }
        copied.set(true);
        toast.success(
            "Reference copied",
            format!("Keep {value} somewhere safe — you'll need it at check-in."),
        );
    };

    view! {
        <button
            on:click=copy
            class="mt-3 inline-flex items-center gap-1.5 rounded-lg bg-white/15 px-3 py-1.5 text-xs font-semibold ring-1 ring-white/25 backdrop-blur transition-colors hover:bg-white/25"
        >
            {move || if copied.get() {
                view! { <Icon name="check" class="h-3.5 w-3.5" /> }.into_any()
            } else {
                view! { <Icon name="file-text" class="h-3.5 w-3.5" /> }.into_any()
            }}
            {move || if copied.get() { "Copied" } else { "Copy reference" }}
        </button>
    }
}

#[component]
fn Step(n: &'static str, title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <li class="flex gap-3">
            <span class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-blue-100 text-xs font-bold text-blue-700">
                {n}
            </span>
            <span class="min-w-0">
                <span class="block text-sm font-semibold text-slate-800">{title}</span>
                <span class="block text-xs leading-relaxed text-slate-500">{body}</span>
            </span>
        </li>
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
