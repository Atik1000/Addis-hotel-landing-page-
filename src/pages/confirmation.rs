use crate::components::{thousands, use_toast, Icon};
use crate::data::{find_hotel, Hotel};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::{use_params_map, use_query_map};

/// Percent-encodes text for use inside a `data:` or `mailto:` URI.
fn uri_encode(value: &str) -> String {
    value
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[component]
pub fn ConfirmationPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();

    let booking_ref =
        move || params.get().get("booking_ref").unwrap_or_else(|| "HA-250143".to_string());

    // The wizard can pass the chosen hotel through; otherwise fall back to the
    // flagship property so the page always has something concrete to show.
    let hotel = move || -> &'static Hotel {
        query
            .get()
            .get("hotel")
            .and_then(|id| find_hotel(&id))
            .or_else(|| find_hotel("golden-tulip-addis-ababa"))
            .unwrap_or(&crate::data::HOTELS[0])
    };

    let toast = use_toast();
    let copied = RwSignal::new(false);

    let ticket_text = move || {
        let h = hotel();
        format!(
            "HORN OF AFRICA HOTEL PORTAL\r\n\
             ===========================\r\n\r\n\
             BOOKING REFERENCE: {}\r\n\r\n\
             Hotel:      {}\r\n\
             Address:    {}\r\n\
             Phone:      {}\r\n\r\n\
             Check-in:   4 September 2026 (from {})\r\n\
             Check-out:  6 September 2026 (by {})\r\n\
             Guests:     2 guests, 1 room\r\n\
             Room:       {}\r\n\r\n\
             Total:      ETB {} (payable at the hotel)\r\n\r\n\
             Present this reference at the front desk.\r\n\
             No payment has been taken online.\r\n",
            booking_ref(),
            h.name,
            h.location_line(),
            h.phone,
            h.check_in_time,
            h.check_out_time,
            h.rooms.first().map(|r| r.name).unwrap_or("Standard Room"),
            thousands(h.price_from * 2 * 115 / 100),
        )
    };

    let download_href =
        move || format!("data:text/plain;charset=utf-8,{}", uri_encode(&ticket_text()));

    let whatsapp_href = move || {
        format!(
            "https://wa.me/?text={}",
            uri_encode(&format!(
                "My reservation at {} is confirmed. Booking reference: {}",
                hotel().name,
                booking_ref()
            ))
        )
    };

    let mail_href = move || {
        format!(
            "mailto:?subject={}&body={}",
            uri_encode(&format!("Reservation {} confirmed", booking_ref())),
            uri_encode(&ticket_text())
        )
    };

    view! {
        <Title text="Reservation confirmed — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-2xl px-4 py-10">
            // ---- Hero ---------------------------------------------------
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
                    "Reservation confirmed"
                </h1>
                <p class="mt-2 max-w-md animate-fade-up text-sm leading-relaxed text-slate-500" style="animation-delay: 170ms">
                    "Your room is held. Show the reference below at the front desk — nothing has been charged online."
                </p>
            </div>

            // ---- Reference ticket ---------------------------------------
            <div class="relative mt-7 animate-fade-up overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-lg shadow-slate-900/5" style="animation-delay: 220ms">
                <div class="bg-gradient-to-br from-blue-700 via-blue-800 to-indigo-900 px-6 py-7 text-center text-white">
                    <p class="text-xs font-semibold uppercase tracking-[0.2em] text-blue-200">"Booking reference"</p>
                    <p class="mt-2 text-4xl font-extrabold tracking-[0.14em] tabular-nums">{booking_ref}</p>
                    <button
                        on:click=move |_| {
                            copied.set(true);
                            toast.success("Reference noted", format!("Keep {} somewhere safe — you'll need it at check-in.", booking_ref()));
                        }
                        class="mt-3 inline-flex items-center gap-1.5 rounded-lg bg-white/15 px-3 py-1.5 text-xs font-semibold ring-1 ring-white/25 backdrop-blur transition-colors hover:bg-white/25"
                    >
                        {move || if copied.get() {
                            view! { <Icon name="check" class="h-3.5 w-3.5" /> }.into_any()
                        } else {
                            view! { <Icon name="file-text" class="h-3.5 w-3.5" /> }.into_any()
                        }}
                        {move || if copied.get() { "Saved to your notes" } else { "Save this reference" }}
                    </button>
                </div>

                // Perforated edge between the header and the details.
                <div class="relative h-4 bg-white">
                    <span class="absolute -left-2 top-1/2 h-4 w-4 -translate-y-1/2 rounded-full bg-slate-50 ring-1 ring-slate-200"></span>
                    <span class="absolute -right-2 top-1/2 h-4 w-4 -translate-y-1/2 rounded-full bg-slate-50 ring-1 ring-slate-200"></span>
                    <span class="absolute inset-x-5 top-1/2 border-t-2 border-dashed border-slate-200"></span>
                </div>

                <div class="px-6 pb-6">
                    {move || {
                        let h = hotel();
                        view! {
                            <div class="flex gap-4">
                                <img src=h.image alt=h.name class="h-20 w-20 shrink-0 rounded-xl object-cover" />
                                <div class="min-w-0">
                                    <p class="text-base font-bold text-slate-900">{h.name}</p>
                                    <p class="flex items-center gap-1 text-xs text-slate-500">
                                        <Icon name="map-pin" class="h-3 w-3" />
                                        {h.location_line()}
                                    </p>
                                    <p class="mt-1 text-xs text-slate-500">
                                        {h.rooms.first().map(|r| format!("{} · {} · Sleeps {}", r.name, r.beds, r.guests)).unwrap_or_default()}
                                    </p>
                                    <a href=format!("tel:{}", h.phone.replace(' ', "")) class="mt-1.5 inline-flex items-center gap-1 text-xs font-bold text-blue-700 hover:underline">
                                        <Icon name="phone" class="h-3 w-3" />
                                        {h.phone}
                                    </a>
                                </div>
                            </div>

                            <dl class="mt-5 grid grid-cols-2 gap-3 border-t border-slate-100 pt-5 text-sm sm:grid-cols-4">
                                <Fact icon="calendar" label="Check-in" value="4 Sep 2026" hint=h.check_in_time />
                                <Fact icon="calendar" label="Check-out" value="6 Sep 2026" hint=h.check_out_time />
                                <Fact icon="moon" label="Nights" value="2" hint="1 room" />
                                <Fact icon="users" label="Guests" value="2" hint="Adults" />
                            </dl>

                            <div class="mt-5 flex items-end justify-between gap-3 rounded-xl bg-slate-50 px-4 py-3.5">
                                <span>
                                    <span class="block text-sm font-bold text-slate-800">"Total payable at hotel"</span>
                                    <span class="block text-xs text-slate-500">"Includes taxes · No booking fee"</span>
                                </span>
                                <span class="text-xl font-extrabold tracking-tight text-slate-900 tabular-nums">
                                    {format!("ETB {}", thousands(h.price_from * 2 * 115 / 100))}
                                </span>
                            </div>
                        }
                    }}
                </div>
            </div>

            // ---- Actions -------------------------------------------------
            <div class="mt-5 grid animate-fade-up grid-cols-2 gap-2.5 sm:grid-cols-4" style="animation-delay: 280ms">
                <a
                    href=download_href
                    download="reservation.txt"
                    class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-300 bg-white py-3.5 text-xs font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="download" class="h-4.5 w-4.5" />
                    "Download"
                </a>
                <a
                    href=whatsapp_href
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-300 bg-white py-3.5 text-xs font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-emerald-300 hover:text-emerald-700 hover:shadow-md"
                >
                    <Icon name="message" class="h-4.5 w-4.5 text-emerald-600" />
                    "WhatsApp"
                </a>
                <a
                    href=mail_href
                    class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-300 bg-white py-3.5 text-xs font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="mail" class="h-4.5 w-4.5" />
                    "Email"
                </a>
                // A raw `onclick` keeps printing dependency-free; the handler is
                // inert during server rendering.
                <button
                    onclick="window.print()"
                    class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-300 bg-white py-3.5 text-xs font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="printer" class="h-4.5 w-4.5" />
                    "Print"
                </button>
            </div>

            // ---- What happens next ---------------------------------------
            <div class="mt-7 animate-fade-up rounded-2xl border border-slate-200 bg-white p-6" style="animation-delay: 330ms">
                <h2 class="text-base font-bold text-slate-900">"What happens next"</h2>
                <ol class="mt-4 flex flex-col gap-0">
                    {[
                        ("check-circle", "Hotel notified", "The property already has your reservation in its dashboard.", true),
                        ("phone", "Optional call", "Ring the hotel if you expect to arrive after midnight so they can hold the room.", false),
                        ("key", "Check in", "Show your reference and ID at the front desk on 4 September.", false),
                        ("wallet", "Pay at the desk", "Settle the bill in cash or by card when you arrive.", false),
                    ].into_iter().enumerate().map(|(i, (icon, title, body, done))| {
                        let is_last = i == 3;
                        view! {
                            <li class="relative flex gap-4 pb-5 last:pb-0">
                                <Show when=move || !is_last>
                                    <span class="absolute left-[15px] top-9 h-[calc(100%-1.5rem)] w-0.5 bg-slate-200"></span>
                                </Show>
                                <span class=format!(
                                    "relative z-10 flex h-8 w-8 shrink-0 items-center justify-center rounded-full ring-4 ring-white {}",
                                    if done { "bg-emerald-500 text-white" } else { "bg-slate-100 text-slate-500" }
                                )>
                                    <Icon name=icon class="h-4 w-4" />
                                </span>
                                <span class="pt-0.5">
                                    <span class="block text-sm font-bold text-slate-800">{title}</span>
                                    <span class="mt-0.5 block text-sm leading-relaxed text-slate-500">{body}</span>
                                </span>
                            </li>
                        }
                    }).collect_view()}
                </ol>
            </div>

            <div class="mt-4 flex animate-fade-up items-start gap-3 rounded-2xl bg-blue-50 p-4 text-sm text-blue-900 ring-1 ring-blue-100" style="animation-delay: 380ms">
                <Icon name="shield-check" class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                <span>
                    <span class="font-bold">"Saved automatically. "</span>
                    "This reservation now appears under My Reservation on this device, and can be pulled up anywhere with your reference and contact details."
                </span>
            </div>

            <div class="mt-6 flex animate-fade-up flex-col gap-2.5 sm:flex-row" style="animation-delay: 430ms">
                <A
                    href="/my-reservations"
                    attr:class="sheen flex flex-1 items-center justify-center gap-2 rounded-xl bg-blue-700 py-3.5 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98]"
                >
                    <Icon name="ticket" class="h-4 w-4" />
                    "View my reservations"
                </A>
                <A
                    href="/"
                    attr:class="flex flex-1 items-center justify-center gap-2 rounded-xl border border-slate-300 py-3.5 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="home" class="h-4 w-4" />
                    "Back to home"
                </A>
            </div>
        </div>
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
