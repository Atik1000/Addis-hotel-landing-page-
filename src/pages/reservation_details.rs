use crate::components::{thousands, use_toast, Breadcrumbs, Icon, Modal};
use crate::data::{find_hotel, Hotel};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ReservationDetailsPage() -> impl IntoView {
    let params = use_params_map();
    let toast = use_toast();
    let cancelling = RwSignal::new(false);
    let cancelled = RwSignal::new(false);

    let reference =
        move || params.get().get("booking_ref").unwrap_or_else(|| "HA-250143".to_string());

    let hotel: &'static Hotel =
        find_hotel("golden-tulip-addis-ababa").unwrap_or(&crate::data::HOTELS[0]);
    let room = hotel.rooms.get(1).or_else(|| hotel.rooms.first());
    let nightly = room.map(|r| r.price_per_night).unwrap_or(hotel.price_from);
    let subtotal = nightly * 2;
    let taxes = subtotal * 15 / 100;

    let do_cancel = move |_| {
        cancelled.set(true);
        cancelling.set(false);
        toast.success(
            "Reservation cancelled",
            format!("{} has been cancelled. The hotel has been notified.", reference()),
        );
    };

    view! {
        <Title text=move || format!("Reservation {} — Horn of Africa Hotel Portal", reference()) />

        <div class="mx-auto max-w-3xl px-4 py-6 pb-24 sm:pb-6">
            <div class="mb-4 animate-fade-up">
                <Breadcrumbs trail=vec![
                    ("Home".to_string(), Some("/".to_string())),
                    ("My Reservation".to_string(), Some("/my-reservations".to_string())),
                    (reference(), None),
                ] />
            </div>

            // ---- Status banner ---------------------------------------------
            <div class=move || format!(
                "flex animate-fade-up items-center gap-3.5 rounded-2xl px-5 py-4 text-white shadow-lg {}",
                if cancelled.get() {
                    "bg-gradient-to-r from-slate-600 to-slate-700 shadow-slate-900/20"
                } else {
                    "bg-gradient-to-r from-emerald-600 to-teal-600 shadow-emerald-900/20"
                }
            )>
                <span class="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-white/20 ring-1 ring-white/25">
                    {move || if cancelled.get() {
                        view! { <Icon name="x-circle" class="h-5 w-5" /> }.into_any()
                    } else {
                        view! { <Icon name="check-circle" class="h-5 w-5" /> }.into_any()
                    }}
                </span>
                <div class="min-w-0">
                    <p class="text-base font-bold">
                        {move || if cancelled.get() { "Reservation cancelled" } else { "Reservation confirmed" }}
                    </p>
                    <p class="text-sm opacity-90">
                        {move || if cancelled.get() {
                            "The room has been released. Nothing was charged.".to_string()
                        } else {
                            format!("Reference {} · Show this at the front desk", reference())
                        }}
                    </p>
                </div>
            </div>

            // ---- Hotel card --------------------------------------------------
            <div class="mt-5 animate-fade-up overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm" style="animation-delay: 80ms">
                <div class="skeleton relative h-44 overflow-hidden">
                    <img src=hotel.image alt=hotel.name class="h-full w-full object-cover" />
                    <span class="absolute inset-0 bg-gradient-to-t from-slate-950/85 via-slate-950/20 to-transparent"></span>
                    <div class="absolute inset-x-5 bottom-4 text-white">
                        <p class="text-lg font-bold">{hotel.name}</p>
                        <p class="flex items-center gap-1 text-xs text-slate-200">
                            <Icon name="map-pin" class="h-3 w-3" />
                            {hotel.location_line()}
                        </p>
                    </div>
                    <A
                        href=format!("/hotels/{}", hotel.id)
                        attr:class="absolute right-4 top-4 rounded-lg bg-white/90 px-3 py-1.5 text-xs font-bold text-slate-800 backdrop-blur transition-transform hover:scale-105"
                    >
                        "View hotel"
                    </A>
                </div>

                <div class="p-5">
                    <dl class="grid grid-cols-2 gap-4 sm:grid-cols-4">
                        <Fact icon="calendar" label="Check-in" value="4 Sep 2026" hint=hotel.check_in_time />
                        <Fact icon="calendar" label="Check-out" value="6 Sep 2026" hint=hotel.check_out_time />
                        <Fact icon="moon" label="Nights" value="2" hint="1 room" />
                        <Fact icon="users" label="Guests" value="2" hint="Adults" />
                    </dl>

                    <div class="mt-5 border-t border-slate-100 pt-5">
                        <h2 class="mb-3 text-sm font-bold text-slate-900">"Room"</h2>
                        {match room {
                            Some(r) => view! {
                                <div class="flex gap-3.5">
                                    <img src=r.image alt=r.name class="h-16 w-16 shrink-0 rounded-xl object-cover" />
                                    <div class="min-w-0">
                                        <p class="text-sm font-bold text-slate-900">{r.name}</p>
                                        <p class="text-xs text-slate-500">{r.beds}</p>
                                        <p class="text-xs text-slate-500">{format!("Sleeps {} · {} m²", r.guests, r.size_sqm)}</p>
                                    </div>
                                </div>
                            }.into_any(),
                            None => view! { <p class="text-sm text-slate-500">"Room details unavailable."</p> }.into_any(),
                        }}
                    </div>

                    <div class="mt-5 border-t border-slate-100 pt-5">
                        <h2 class="mb-3 text-sm font-bold text-slate-900">"Lead guest"</h2>
                        <dl class="flex flex-col gap-2 text-sm">
                            <Row label="Name" value="Ahmed Hassan" />
                            <Row label="Contact" value="+251 91 234 5678" />
                            <Row label="Email" value="ahmed.hassan@example.com" />
                            <Row label="Arrival" value="Afternoon (12:00 – 18:00)" />
                        </dl>
                    </div>

                    <div class="mt-5 border-t border-slate-100 pt-5">
                        <h2 class="mb-3 text-sm font-bold text-slate-900">"Price breakdown"</h2>
                        <dl class="flex flex-col gap-2 text-sm">
                            <div class="flex justify-between">
                                <dt class="text-slate-500">{format!("ETB {} × 2 nights", thousands(nightly))}</dt>
                                <dd class="font-semibold text-slate-800 tabular-nums">{thousands(subtotal)}</dd>
                            </div>
                            <div class="flex justify-between">
                                <dt class="text-slate-500">"Taxes & tourism levy (15%)"</dt>
                                <dd class="font-semibold text-slate-800 tabular-nums">{thousands(taxes)}</dd>
                            </div>
                            <div class="flex justify-between">
                                <dt class="text-slate-500">"Booking fee"</dt>
                                <dd class="font-bold text-emerald-600">"FREE"</dd>
                            </div>
                            <div class="mt-2 flex items-end justify-between border-t border-slate-200 pt-3">
                                <dt class="font-bold text-slate-800">"Total at the hotel"</dt>
                                <dd class="text-xl font-extrabold text-slate-900 tabular-nums">
                                    {format!("ETB {}", thousands(subtotal + taxes))}
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>

            // ---- Contact -----------------------------------------------------
            <div class="mt-4 grid animate-fade-up gap-2.5 sm:grid-cols-3" style="animation-delay: 140ms">
                <a
                    href=format!("tel:{}", hotel.phone.replace(' ', ""))
                    class="flex items-center justify-center gap-2 rounded-xl border border-slate-300 bg-white py-3 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="phone" class="h-4 w-4" />
                    "Call hotel"
                </a>
                <a
                    href=format!("https://wa.me/{}", hotel.whatsapp.replace([' ', '+'], ""))
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center gap-2 rounded-xl border border-slate-300 bg-white py-3 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-emerald-300 hover:text-emerald-700 hover:shadow-md"
                >
                    <Icon name="message" class="h-4 w-4 text-emerald-600" />
                    "WhatsApp"
                </a>
                <a
                    href=format!(
                        "https://www.google.com/maps/search/?api=1&query={}",
                        hotel.location_line().replace(' ', "+")
                    )
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center gap-2 rounded-xl border border-slate-300 bg-white py-3 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="map" class="h-4 w-4" />
                    "Directions"
                </a>
            </div>

            // ---- Danger zone --------------------------------------------------
            <Show when=move || !cancelled.get()>
                <div class="mt-5 animate-fade-up rounded-2xl border border-slate-200 bg-white p-5" style="animation-delay: 200ms">
                    <h2 class="text-sm font-bold text-slate-900">"Need to change plans?"</h2>
                    <p class="mt-1 text-sm leading-relaxed text-slate-500">
                        "This rate can be cancelled free of charge up to 24 hours before check-in. To change your dates instead, call the hotel directly — they can usually move a reservation without cancelling it."
                    </p>
                    <button
                        on:click=move |_| cancelling.set(true)
                        class="mt-3.5 flex items-center gap-2 rounded-xl border border-red-200 px-4 py-2.5 text-sm font-bold text-red-600 transition-all duration-200 hover:-translate-y-0.5 hover:border-red-300 hover:bg-red-50"
                    >
                        <Icon name="x-circle" class="h-4 w-4" />
                        "Cancel reservation"
                    </button>
                </div>
            </Show>

            <A
                href="/my-reservations"
                attr:class="mt-5 flex items-center justify-center gap-2 rounded-xl border border-slate-300 py-3 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50"
            >
                <Icon name="arrow-left" class="h-4 w-4" />
                "All reservations"
            </A>
        </div>

        <Show when=move || cancelling.get()>
            <Modal title="Cancel this reservation?" width="max-w-md" on_close=move || cancelling.set(false)>
                <div class="flex items-start gap-3 rounded-xl bg-amber-50 p-4 text-sm text-amber-900 ring-1 ring-amber-100">
                    <Icon name="alert" class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                    <span>"The room will be released immediately and this cannot be undone."</span>
                </div>
                <div class="mt-6 flex gap-2.5">
                    <button
                        on:click=move |_| cancelling.set(false)
                        class="flex-1 rounded-xl border border-slate-300 py-3 text-sm font-bold text-slate-700 transition-colors hover:bg-slate-50"
                    >
                        "Keep it"
                    </button>
                    <button
                        on:click=do_cancel
                        class="flex-1 rounded-xl bg-red-600 py-3 text-sm font-bold text-white shadow-lg shadow-red-600/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-red-700"
                    >
                        "Cancel reservation"
                    </button>
                </div>
            </Modal>
        </Show>
    }
}

#[component]
fn Fact(
    icon: &'static str,
    label: &'static str,
    value: &'static str,
    hint: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <dt class="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wide text-slate-400">
                <Icon name=icon class="h-3 w-3" />
                {label}
            </dt>
            <dd class="mt-0.5 text-sm font-bold text-slate-900">{value}</dd>
            <dd class="text-xs text-slate-400">{hint}</dd>
        </div>
    }
}

#[component]
fn Row(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between gap-4">
            <dt class="text-slate-500">{label}</dt>
            <dd class="text-right font-semibold text-slate-800">{value}</dd>
        </div>
    }
}
